pub enum PopupMsg {
    Show,
    Search(Option<String>),
    Accept(String),
}