use std::time::{self, Duration};

use fake::{Fake, Faker};
use gpui::{
    div, impl_actions, px, AnyElement, AppContext, Edges, InteractiveElement, IntoElement,
    ParentElement, Pixels, Render, SharedString, Styled, Timer, View, ViewContext,
    VisualContext as _, WindowContext,
};
use serde::Deserialize;
use ui::{
    input::{InputEvent, TextInput},
    popup_menu::PopupMenu,
    table::{ColFixed, ColSort, Table, TableDelegate, TableEvent},
    v_flex, Size, StyleSized as _,
};

#[derive(Clone, PartialEq, Eq, Deserialize)]
struct ChangeSize(Size);

impl_actions!(table_story, [ChangeSize]);

#[derive(Clone, Debug, Default)]
struct Stock {
    id: usize,
    symbol: String,
    name: String,
    price: f64,
}

impl Stock {
    fn random_update(&mut self) {
        self.price = (-300.0..999.999).fake::<f64>();
    }
}

fn random_stocks(size: usize) -> Vec<Stock> {
    (0..size)
        .map(|id| Stock {
            id,
            symbol: Faker.fake::<String>(),
            name: Faker.fake::<String>(),
            ..Default::default()
        })
        .collect()
}

struct Column {
    id: SharedString,
    name: SharedString,
}

impl Column {
    fn new(
        id: impl Into<SharedString>,
        name: impl Into<SharedString>,
        _sort: Option<ColSort>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }
}

struct StockTableDelegate {
    stocks: Vec<Stock>,
    columns: Vec<Column>,
    size: Size,
}

impl StockTableDelegate {
    fn new(size: usize) -> Self {
        Self {
            size: Size::default(),
            stocks: random_stocks(size),
            columns: vec![
                Column::new("id", "ID", None),
                Column::new("symbol", "Symbol", Some(ColSort::Default)),
                Column::new("name", "Name", None),
                Column::new("price", "Price", Some(ColSort::Default)),
            ],
        }
    }

    fn update_stocks(&mut self, size: usize) {
        self.stocks = random_stocks(size);
    }

    fn render_value_cell(&self, val: f64) -> AnyElement {
        let this = div()
            .h_full()
            .table_cell_size(self.size)
            .child(format!("{:.3}", val));
        // Val is a 0.0 .. n.0
        // 30% to red, 30% to green, others to default
        let right_num = ((val - val.floor()) * 1000.).floor() as i32;

        let this = if right_num % 3 == 0 {
            this.text_color(ui::red_600()).bg(ui::red_50().opacity(0.6))
        } else if right_num % 3 == 1 {
            this.text_color(ui::green_600())
                .bg(ui::green_50().opacity(0.6))
        } else {
            this
        };

        this.into_any_element()
    }
}

impl TableDelegate for StockTableDelegate {
    fn cols_count(&self, _: &AppContext) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _: &AppContext) -> usize {
        self.stocks.len()
    }

    fn col_name(&self, col_ix: usize, _: &AppContext) -> SharedString {
        if let Some(col) = self.columns.get(col_ix) {
            col.name.clone()
        } else {
            "--".into()
        }
    }

    fn col_width(&self, col_ix: usize, _: &AppContext) -> Option<Pixels> {
        if let Some(_) = self.columns.get(col_ix) {
            Some(120.0.into())
        } else {
            None
        }
    }

    fn col_padding(&self, col_ix: usize, _: &AppContext) -> Option<Edges<Pixels>> {
        if col_ix >= 3 && col_ix <= 10 {
            Some(Edges::all(px(0.)))
        } else {
            None
        }
    }

    fn col_fixed(&self, col_ix: usize, _: &AppContext) -> Option<ui::table::ColFixed> {
        if col_ix < 4 {
            Some(ColFixed::Left)
        } else {
            None
        }
    }

    fn render_th(&self, col_ix: usize, cx: &mut ViewContext<Table<Self>>) -> impl IntoElement {
        let th = div().child(self.col_name(col_ix, cx));

        if col_ix >= 3 && col_ix <= 10 {
            th.table_cell_size(self.size)
        } else {
            th
        }
    }

    fn context_menu(&self, _: usize, menu: PopupMenu, _: &WindowContext) -> PopupMenu {
        menu.menu("Size Large", Box::new(ChangeSize(Size::Large)))
            .menu("Size Medium", Box::new(ChangeSize(Size::Medium)))
            .menu("Size Small", Box::new(ChangeSize(Size::Small)))
            .menu("Size XSmall", Box::new(ChangeSize(Size::XSmall)))
    }

    fn render_td(
        &self,
        row_ix: usize,
        col_ix: usize,
        _cx: &mut ViewContext<Table<Self>>,
    ) -> impl IntoElement {
        let stock = self.stocks.get(row_ix).unwrap();
        let col = self.columns.get(col_ix).unwrap();

        match col.id.as_ref() {
            "id" => stock.id.to_string().into_any_element(),
            "name" => stock.name.clone().into_any_element(),
            "symbol" => stock.symbol.clone().into_any_element(),
            "price" => self.render_value_cell(stock.price),
            _ => "--".to_string().into_any_element(),
        }
    }

    fn move_col(&mut self, col_ix: usize, to_ix: usize, _: &mut ViewContext<Table<Self>>) {
        let col = self.columns.remove(col_ix);
        self.columns.insert(to_ix, col);
    }

    fn load_more_threshold(&self) -> usize {
        150
    }

    fn load_more(&mut self, cx: &mut ViewContext<Table<Self>>) {
        cx.spawn(|view, mut cx| async move {
            // Simulate network request, delay 1s to load data.
            Timer::after(Duration::from_secs(1)).await;

            cx.update(|cx| {
                let _ = view.update(cx, |view, _| {
                    view.delegate_mut().stocks.extend(random_stocks(200));
                });
            })
        })
        .detach();
    }
}

pub struct TableStory {
    table: View<Table<StockTableDelegate>>,
    num_stocks_input: View<TextInput>,
    refresh_data: bool,
    size: Size,
}

impl super::Story for TableStory {
    fn title() -> &'static str {
        "Table"
    }

    fn description() -> &'static str {
        "A complex data table with selection, sorting, column moving, and loading more."
    }

    fn new_view(cx: &mut WindowContext) -> View<impl gpui::FocusableView> {
        Self::view(cx)
    }

    fn closeable() -> bool {
        false
    }
}

impl gpui::FocusableView for TableStory {
    fn focus_handle(&self, cx: &gpui::AppContext) -> gpui::FocusHandle {
        self.table.focus_handle(cx)
    }
}

impl TableStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(Self::new)
    }

    fn new(cx: &mut ViewContext<Self>) -> Self {
        // Create the number input field with validation for positive integers
        let num_stocks_input = cx.new_view(|cx| {
            let mut input = TextInput::new(cx)
                .placeholder("Enter number of Stocks to display")
                .validate(|s| s.parse::<usize>().is_ok());
            input.set_text("5", cx);
            input
        });

        let delegate = StockTableDelegate::new(5);
        let table = cx.new_view(|cx| Table::new(delegate, cx));

        cx.subscribe(&table, Self::on_table_event).detach();
        cx.subscribe(&num_stocks_input, Self::on_num_stocks_input_change)
            .detach();

        // Spawn a background to random refresh the list
        cx.spawn(move |this, mut cx| async move {
            loop {
                let delay = (80..150).fake::<u64>();
                Timer::after(time::Duration::from_millis(delay)).await;

                this.update(&mut cx, |this, cx| {
                    if !this.refresh_data {
                        return;
                    }

                    this.table.update(cx, |table, _| {
                        table.delegate_mut().stocks.iter_mut().enumerate().for_each(
                            |(i, stock)| {
                                let n = (3..10).fake::<usize>();
                                // update 30% of the stocks
                                if i % n == 0 {
                                    stock.random_update();
                                }
                            },
                        );
                    });
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();

        Self {
            table,
            num_stocks_input,
            refresh_data: false,
            size: Size::default(),
        }
    }

    // Event handler for changes in the number input field
    fn on_num_stocks_input_change(
        &mut self,
        _: View<TextInput>,
        event: &InputEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            // Update when the user presses Enter or the input loses focus
            InputEvent::PressEnter | InputEvent::Blur => {
                let text = self.num_stocks_input.read(cx).text().to_string();
                if let Ok(num) = text.parse::<usize>() {
                    self.table.update(cx, |table, _| {
                        table.delegate_mut().update_stocks(num);
                    });
                    cx.notify();
                }
            }
            _ => {}
        }
    }

    fn on_change_size(&mut self, a: &ChangeSize, cx: &mut ViewContext<Self>) {
        self.size = a.0;
        self.table.update(cx, |table, cx| {
            table.set_size(a.0, cx);
            table.delegate_mut().size = a.0;
        });
    }

    fn on_table_event(
        &mut self,
        _: View<Table<StockTableDelegate>>,
        event: &TableEvent,
        _cx: &mut ViewContext<Self>,
    ) {
        match event {
            TableEvent::ColWidthsChanged(col_widths) => {
                println!("Col widths changed: {:?}", col_widths)
            }
            TableEvent::SelectCol(ix) => println!("Select col: {}", ix),
            TableEvent::SelectRow(ix) => println!("Select row: {}", ix),
        }
    }
}

impl Render for TableStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl gpui::IntoElement {
        v_flex()
            .on_action(cx.listener(Self::on_change_size))
            .size_full()
            .text_sm()
            .gap_2()
            .child(self.table.clone())
    }
}
