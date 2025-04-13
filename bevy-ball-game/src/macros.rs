#[macro_export]
macro_rules! button_interaction {
    ($type:ty) => {
        Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<$type>)>
    };
}
