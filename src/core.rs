#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FrameflowRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeId {
    pub index: usize,
    pub generation: u32,
}

impl NodeId {
    pub fn new(index: usize, generation: u32) -> Self {
        NodeId { index, generation }
    }

    pub fn invalid() -> Self {
        NodeId { index: usize::MAX, generation: 0 }
    }
}

#[derive(Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub enum Alignment {
    Start,
    Center,
    End,
    SpaceBetween,
}

#[derive(Debug)]
pub struct Anchors {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug)]
pub struct Offsets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug)]
pub enum NodeType {
    Generic,
    Center,
    Box {},
    Split {},
    Margin {
        top: f32,
        right: f32,
        bottom: f32,
        left: f32,
    },
    Flow {
        direction: Direction,
    },
}

#[derive(Debug)]
pub struct Node {
    pub bounds: FrameflowRect,
    pub minimum_size: Float2,

    pub expand: Float2,
    pub stretch: Float2,

    pub anchors: Anchors,
    pub offsets: Offsets,

    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,

    // pub component_index: usize,

    pub node_type: NodeType,
    pub generation: u32,
    pub alive: bool,
}

#[derive(Debug)]
pub struct System {
    alloc: Vec<Node>,
    pub roots: Vec<NodeId>,
    free_list: Vec<usize>,
}

impl Node {
    pub fn new(node_type: NodeType) -> Self {
        Node {
            bounds: FrameflowRect {
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.0,
            },
            minimum_size: Float2 { x: 0.0, y: 0.0 },
            expand: Float2 { x: 1.0, y: 1.0 },
            stretch: Float2 { x: 1.0, y: 1.0 },
            anchors: Anchors {
                top: 0.0,
                right: 1.0,
                bottom: 1.0,
                left: 0.0,
            },
            offsets: Offsets {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            },
            parent: None,
            children: Vec::new(),
            node_type,
            generation: 0,
            alive: true,
        }
    }
}

impl System {
    pub fn new() -> Self {
        System {
            alloc: Vec::new(),
            roots: Vec::new(),
            free_list: Vec::new(),
        }
    }
}

pub fn allocate_node(system: &mut System, node_type: NodeType) -> NodeId {
    if let Some(index) = system.free_list.pop() {
        NodeId::new(index, 0)
    } else {
        let index = system.alloc.len();
        system.alloc.push(Node::new(node_type));
        NodeId::new(index, 0)
    }
}

pub fn get_node(system: &mut System, id: NodeId) -> Option<&mut Node> {
    if id.index >= system.alloc.len() {
        return None;
    }

    let node = &mut system.alloc[id.index];
    Some(node)
}

pub fn is_valid(system: &mut System, id: NodeId) -> bool {
    if id.index == usize::MAX {
        return false;
    }
    if id.index >= system.alloc.len() {
        return false;
    }

    let node = &system.alloc[id.index];
    node.alive && node.generation == id.generation
}

pub fn add_generic(system: &mut System, parent: NodeId) -> Result<NodeId, ()> {
    if !is_valid(system, parent) { return Err(()); }
    let id = allocate_node(system, NodeType::Generic);
    let node = &mut system.alloc[id.index];

    node.parent = Some(parent);

    system.alloc[parent.index].children.push(id);

    Ok(id)
}

