// ANCHOR: app
use crate::capabilities::database::{Database, DatabaseOutput};
use crux_core::{render::Render, App};
use serde::{Deserialize, Serialize};
use uniffi::deps::log::info;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    Initialize,
    Increment,
    Decrement,
    Reset,
    #[serde(skip)]
    DatabaseResponse(DatabaseOutput),
}

#[derive(Default)]
pub struct Model {
    count: isize,
    error: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ViewModel {
    pub count: String,
    pub message: String,
}

#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
pub struct Capabilities {
    render: Render<Event>,
    database: Database<Event>,
}

#[derive(Default)]
pub struct Counter;

// ANCHOR: impl_app
impl App for Counter {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Capabilities = Capabilities;

    fn update(&self, event: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        model.error = "".to_string();
        match event {
            Event::Initialize => {
                info!("Initializing database");
                caps.database.fetch_counter(Event::DatabaseResponse);
            }
            Event::Increment => {
                model.count += 1;
                caps.database
                    .update_counter(model.count, Event::DatabaseResponse);
            }
            Event::Decrement => {
                model.count -= 1;
                caps.database
                    .update_counter(model.count, Event::DatabaseResponse);
            }
            Event::Reset => {
                model.count = 0;
                caps.database
                    .update_counter(model.count, Event::DatabaseResponse);
            }
            Event::DatabaseResponse(database_output) => match database_output {
                DatabaseOutput::Succeeded => println!("Database operation succeeded"),
                DatabaseOutput::Counter(counter) => model.count = counter,
                DatabaseOutput::Failed(e) => model.error = e,
            },
        };
        caps.render.render();
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            count: format!("Count is: {}", model.count),
            message: model.error.clone(),
        }
    }
}
// ANCHOR_END: impl_app
// ANCHOR_END: app

// ANCHOR: test
#[cfg(test)]
mod test {
    use super::*;
    use crux_core::{assert_effect, testing::AppTester};

    #[test]
    fn renders() {
        let app = AppTester::<Counter, _>::default();
        let mut model = Model::default();

        let update = app.update(Event::Reset, &mut model);

        // Check update asked us to `Render`
        assert_effect!(update, Effect::Render(_));
    }

    #[test]
    fn shows_initial_count() {
        let app = AppTester::<Counter, _>::default();
        let model = Model::default();

        let actual_view = app.view(&model).count;
        let expected_view = "Count is: 0";
        assert_eq!(actual_view, expected_view);
    }

    #[test]
    fn increments_count() {
        let app = AppTester::<Counter, _>::default();
        let mut model = Model::default();

        let update = app.update(Event::Increment, &mut model);

        let actual_view = app.view(&model).count;
        let expected_view = "Count is: 1";
        assert_eq!(actual_view, expected_view);

        // Check update asked us to `Render`
        assert_effect!(update, Effect::Render(_));
    }

    #[test]
    fn decrements_count() {
        let app = AppTester::<Counter, _>::default();
        let mut model = Model::default();

        let update = app.update(Event::Decrement, &mut model);

        let actual_view = app.view(&model).count;
        let expected_view = "Count is: -1";
        assert_eq!(actual_view, expected_view);

        // Check update asked us to `Render`
        assert_effect!(update, Effect::Render(_));
    }

    #[test]
    fn resets_count() {
        let app = AppTester::<Counter, _>::default();
        let mut model = Model::default();

        let _ = app.update(Event::Increment, &mut model);
        let _ = app.update(Event::Reset, &mut model);

        let actual_view = app.view(&model).count;
        let expected_view = "Count is: 0";
        assert_eq!(actual_view, expected_view);
    }

    #[test]
    fn counts_up_and_down() {
        let app = AppTester::<Counter, _>::default();
        let mut model = Model::default();

        let _ = app.update(Event::Increment, &mut model);
        let _ = app.update(Event::Reset, &mut model);
        let _ = app.update(Event::Decrement, &mut model);
        let _ = app.update(Event::Increment, &mut model);
        let _ = app.update(Event::Increment, &mut model);

        let actual_view = app.view(&model).count;
        let expected_view = "Count is: 1";
        assert_eq!(actual_view, expected_view);
    }
}
// ANCHOR_END: test
