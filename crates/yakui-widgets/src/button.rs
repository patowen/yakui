use yakui_core::{
    context::Context,
    dom::{Dom, LayoutDom},
    draw, Color3, Component, ComponentEvent, Constraints, Index, MouseButton, Vec2,
};

#[derive(Debug)]
pub struct Button {
    index: Index,
    props: ButtonProps,
    hovering: bool,
    mouse_down: bool,
    clicked: bool,
}

#[derive(Debug, Clone)]
pub struct ButtonProps {
    pub size: Vec2,
    pub fill: Color3,
    pub hover_fill: Option<Color3>,
    pub down_fill: Option<Color3>,
}

impl ButtonProps {
    pub fn unstyled<S: Into<Vec2>>(size: S) -> Self {
        Self {
            size: size.into(),
            fill: Color3::GRAY,
            hover_fill: None,
            down_fill: None,
        }
    }

    pub fn styled<S: Into<Vec2>>(size: S) -> Self {
        Self {
            size: size.into(),
            fill: Color3::rgb(50, 94, 168),
            hover_fill: Some(Color3::rgb(88, 129, 199)),
            down_fill: Some(Color3::rgb(30, 76, 156)),
        }
    }
}

#[derive(Debug)]
pub struct ButtonResponse {
    pub hovering: bool,
    pub clicked: bool,
}

impl Component for Button {
    type Props = ButtonProps;
    type Response = ButtonResponse;

    fn new(index: Index, props: Self::Props) -> Self {
        Self {
            index,
            props,
            hovering: false,
            mouse_down: false,
            clicked: false,
        }
    }

    fn update(&mut self, props: &Self::Props) {
        self.props = props.clone();
    }

    fn size(&self, _dom: &Dom, _layout: &mut LayoutDom, constraints: Constraints) -> Vec2 {
        constraints.constrain(self.props.size)
    }

    fn draw(&self, _dom: &Dom, layout: &LayoutDom, output: &mut draw::Output) {
        let node = layout.get(self.index).unwrap();
        let viewport = layout.viewport;
        let size = node.rect.size() / viewport.size();
        let pos = (node.rect.pos() + viewport.pos()) / viewport.size();

        let mut color = self.props.fill;

        if let (Some(fill), true) = (self.props.down_fill, self.mouse_down) {
            color = fill
        } else if let (Some(hover), true) = (self.props.hover_fill, self.hovering) {
            color = hover
        }

        output.rect(pos, size, color);
    }

    fn respond(&mut self) -> Self::Response {
        let clicked = self.clicked;
        self.clicked = false;

        Self::Response {
            hovering: self.hovering,
            clicked,
        }
    }

    fn event(&mut self, event: &ComponentEvent) {
        match event {
            ComponentEvent::MouseEnter => {
                self.hovering = true;
            }
            ComponentEvent::MouseLeave => {
                self.hovering = false;
            }
            ComponentEvent::MouseButtonChangedInside(MouseButton::One, down) => {
                if *down {
                    self.mouse_down = true;
                } else if self.mouse_down {
                    self.mouse_down = false;
                    self.clicked = true;
                }
            }
            _ => {}
        }
    }
}

pub fn button(props: ButtonProps) -> ButtonResponse {
    let context = Context::active();
    let res = context.borrow_mut().dom_mut().do_component::<Button>(props);
    res
}