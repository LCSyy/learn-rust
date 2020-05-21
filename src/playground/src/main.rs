/*
---> View  --> Controller
|              |
|              |
|              |
<--- Model <----
*/
struct View {
    controller: Controller,
    model: Model
}

struct Controller {}
struct Model {}

impl View {
    fn onClicked(&self) {
        self.controller.processClick(&self.model);
    }
}

impl Controller {
    fn processClick(&self, model: &Model) {
        // preprcess
        model.setData();
    }
}

impl Model {
    fn setData(&self) {

    }
}

fn main() {

}