pub(self)

struct RenderElement {
    
}
impl RenderElement {
    
}

#[derive(PartialEq)]
pub enum Node {
    Blend {
        layers: Vec<Node>,
    },
    Circle {
        radius: f32,
        transform: glam::Mat3,
    }
}



