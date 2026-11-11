use crate::core::*;

pub fn layout_generic(system: &mut System, parent: NodeId) -> NodeId {
    let id = allocate_node(system);
    id
}

pub fn compute_layout(system: &mut System, node_id: NodeId) {
    let Some(node) = get_node(system, node_id) else {
        return;
    };

    match &node.node_type {
        NodeType::Generic => {
            layout_generic(system, node_id);
        }
        NodeType::Center => {}
        NodeType::Box {} => {}
        NodeType::Split {} => {}
        NodeType::Margin {
            top,
            right,
            bottom,
            left,
        } => {}
        NodeType::Flow { direction } => {}
    }
}
