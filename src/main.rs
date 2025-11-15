mod editor;

use crate::editor::Editor;

fn main() {
    let editor = Editor::default();

    editor.run();
}
