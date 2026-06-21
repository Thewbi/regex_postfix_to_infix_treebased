/*
// Source - https://stackoverflow.com/a/36168774
// Posted by DK.
// Retrieved 2026-06-20, License - CC BY-SA 3.0

use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Note: do not derive Clone to make this move-only.
pub struct Node3(Rc<RefCell<Node3_>>);

pub type WeakNode3 = Weak<RefCell<Node3_>>;

pub struct Node3_ {
    parent: Option<WeakNode3>,
    children: Vec<Node3>,
}

impl Node3 {
    pub fn add(&self, node: Node3) {
        // No need to remove from old parent; 
        // move semantics mean that must have already been done.
        (node.0).borrow_mut().parent = Some(Rc::downgrade(&self.0));
        (node.0).borrow_mut().children.push(node);
    }
}
*/

/*
#[derive(Debug)]
struct Node<T> {
    data: T,
    children: Vec<&mut Node<T>>,
}

impl<T> Node<T> {

    fn new(data: T) -> Node<T> {
        Node { data: data, children: vec![] }
    }

    // fn add_child(&mut self, child: Node<T>) {
    //     self.children.push(child);
    // }

    fn add_child_mut(&mut self, child: &mut Node<T>) {
        self.children.push(child);
    }
}
*/

use std::fmt;

#[derive(Clone, Copy)]
pub struct NodeId {
    index: usize,
}

pub struct Node<T> {
    parent: Option<NodeId>,
    left: Option<NodeId>,
    right: Option<NodeId>,

    // payload
    pub data: T,
}

pub struct Arena<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Arena<T> {

    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.len() == 0
    }

    pub fn new_node(&mut self, data: T) -> NodeId {

        // Get the next free index
        let next_index = self.nodes.len();

        // Push the node into the arena
        self.nodes.push(Node {
            parent: None,
            left: None,
            right: None,
            data: data,
        });

        // Return the node identifier
        NodeId { index: next_index }
    }

    pub fn change_payload(&mut self, node_id: &NodeId, data: T) {
        self.nodes[node_id.index].data = data;
    }

    pub fn get_payload(&mut self, node_id: &NodeId) -> &T {
        &self.nodes[node_id.index].data
    }

    // pub fn get_left_id(&mut self, parent_node_id: &NodeId) -> NodeId {
    //     let parent_node: &Node<T> = &self.nodes[parent_node_id.index];
    //     parent_node.left.as_ref().unwrap().clone()
    // }

    pub fn get_left_id(&mut self, parent_node_id: &NodeId) -> Option<&NodeId> {
        let parent_node: &Node<T> = &self.nodes[parent_node_id.index];
        parent_node.left.as_ref()
    }

    pub fn get_right_id(&mut self, parent_node_id: &NodeId) -> Option<&NodeId> {
        let parent_node: &Node<T> = &self.nodes[parent_node_id.index];
        parent_node.right.as_ref()
    }

    pub fn add_left(&mut self, parent_node_id: &NodeId, data: T) {
        let new_node_id: NodeId = self.new_node(data);
        let parent_node: &mut Node<T> = &mut self.nodes[parent_node_id.index];
        parent_node.left = Some(new_node_id);
    }

    pub fn add_right(&mut self, parent_node_id: &NodeId, data: T) -> NodeId {
        let new_node_id: NodeId = self.new_node(data);
        let parent_node: &mut Node<T> = &mut self.nodes[parent_node_id.index];
        parent_node.right = Some(new_node_id);
        new_node_id.clone()
    }

    pub fn insert_left(&mut self, parent_node_id: &NodeId, left_node_id: NodeId) {
        let parent_node: &mut Node<T> = &mut self.nodes[parent_node_id.index];
        parent_node.left = Some(left_node_id);
    }

    pub fn insert_right(&mut self, parent_node_id: &NodeId, right_node_id: NodeId) {
        let parent_node: &mut Node<T> = &mut self.nodes[parent_node_id.index];
        parent_node.right = Some(right_node_id);
    }

    pub fn insert_repeat_node_into_node(&mut self, node_id: &NodeId, regexBB: T) {

        // Get the next free index
        let next_index = self.nodes.len();

        // Push the node into the arena
        self.nodes.push(Node {
            parent: None,
            left: None,
            right: None,
            data: regexBB,
        });

        //let right_node_id = self.get_right_id(&node_id);

        //let old_right = self.nodes[node_id.index].right.as_ref().unwrap().index;
        let old_right_option = self.nodes[node_id.index].right;

        //self.insert_right(&node_id, repeat_node_id.clone());
        self.nodes[node_id.index].right = Some ( NodeId { index: next_index } );
        
        //self.insert_left(&repeat_node_id, right_node_id.unwrap().clone());
        match old_right_option {
            Some(old_right_node_id) => {
                self.nodes[next_index].left = Some ( NodeId { index: old_right_node_id.index } );
            }
            _ => {

            }
        }
    }

    
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum RegexBuildingBlock {

    CharacterLiteral(char),

    Concatenation,

    CharacterClass(char, char),

    Repeat(u8, u8),

    Or,

    OpeningBraces,
    ClosingBraces,
    ClosedBraces,

}


impl fmt::Debug for RegexBuildingBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CharacterLiteral(c) => {
                //f.debug_tuple("CharacterLiteral").field(c).finish(),
                write!(f, "{}", c);
                Ok(())
            }
            Self::Concatenation => write!(f, "#"),
            Self::CharacterClass(start, end) => {
                //f.debug_tuple("CharacterClass").field(start).field(end).finish(),
                write!(f, "[{}-{}]", start, end);
                Ok(())
            }
            Self::Repeat(min, max) => {
                //f.debug_tuple("Repeat").field(min).field(max).finish(),
                if *min == 0 && *max == 1 {
                    write!(f, "?");
                } else if *min == 0 && *max == std::u8::MAX {
                    write!(f, "*");
                } else if *min == 1 && *max == std::u8::MAX {
                    write!(f, "+");
                } else if *min == *max {
                    write!(f, "{{{}}}", *min);
                } else {
                    write!(f, "{{{},{}}}", *min, *max);
                }
                Ok(())
            }
            Self::Or => write!(f, "|"),

            Self::OpeningBraces => write!(f, "("),
            Self::ClosingBraces => write!(f, ")"),

            //Self::ClosedBraces => write!(f, "()"),
            Self::ClosedBraces => write!(f, ""),
        }
    }
}

pub fn recurse<T>(arena: &Arena<T>, parent_node_id: &NodeId) 
where T:std::fmt::Debug,
{
    let parent_node: &Node<T> = &arena.nodes[parent_node_id.index];
    match &parent_node.left {
        Some(_) => {
            recurse(arena, parent_node.left.as_ref().unwrap());
        }
        None => {
        }
    }
    match &parent_node.right {
        Some(_) => {
            recurse(arena, parent_node.right.as_ref().unwrap());
        }
        None => {
        }
    }
    print!("{:?}", parent_node.data);
}

pub fn recurse_postfix<T>(arena: &Arena<T>, parent_node_id: &NodeId, string_buffer: &mut String) 
where T:std::fmt::Debug,
{
    let parent_node: &Node<T> = &arena.nodes[parent_node_id.index];
    match &parent_node.left {
        Some(_) => {
            recurse_postfix(arena, parent_node.left.as_ref().unwrap(), string_buffer);
        }
        None => {
        }
    }
    match &parent_node.right {
        Some(_) => {
            recurse_postfix(arena, parent_node.right.as_ref().unwrap(), string_buffer);
        }
        None => {
        }
    }
    print!("{:?}", parent_node.data);

    string_buffer.push_str(format!("{:?}", parent_node.data).as_str());
}

pub fn descend_into_concatenation_right_side(arena: &Arena<RegexBuildingBlock>, start_node_id: &NodeId) -> NodeId {
    let mut node_id = start_node_id.clone();
    loop {
        let node: &Node<RegexBuildingBlock> = &arena.nodes[node_id.index];
        match node.right {
            Some(right_node_id) => {
                let right_child_node: &Node<RegexBuildingBlock> = &arena.nodes[right_node_id.index];
                match right_child_node.data {
                    RegexBuildingBlock::Concatenation => {
                        node_id = right_node_id.clone();
                    }
                    _ => {
                        //node_id.clone()
                        break;
                    }
                }
            }
            None => {
                //node_id.clone()
                break;
            }
        }
    }

    node_id
}

pub fn concat_right_side(arena: &mut Arena<RegexBuildingBlock>, node_id: &NodeId, regexBB: RegexBuildingBlock) -> usize {

    // Get the next free index
    let next_index = arena.nodes.len();

    // Push the node into the arena
    arena.nodes.push(Node {
        parent: None,
        left: None,
        right: None,
        data: regexBB,
    });

    // Get the next free index
    let concat_index = arena.nodes.len();

    // Push the node into the arena
    arena.nodes.push(Node {
        parent: None,
        left: None,
        right: None,
        data: RegexBuildingBlock::Concatenation,
    });

    // save right index
    let old_right_option = arena.nodes[node_id.index].right;

    // insert the new concat into the right side
    arena.nodes[node_id.index].right = Some ( NodeId { index: concat_index } );
    
    // insert old right side into left side of the new concat
    match old_right_option {
        Some(old_right_node_id) => {
            arena.nodes[concat_index].left = Some ( NodeId { index: old_right_node_id.index } );
        }
        _ => {

        }
    }

    // insert new building block into the right side of the new concat
    arena.nodes[concat_index].right = Some ( NodeId { index: next_index } );

    next_index
}

pub fn infix_to_postfix(regex_infix: &str) -> String {

    let mut arena: Arena<RegexBuildingBlock> = Arena::new();
    // let mut root_node_id: NodeId = arena.new_node(1);
    let mut root_node_id: NodeId = NodeId {
        index: 0
    };

    let mut character_class_mode: bool = false; // for [a-z]
    let mut repeat_mode: bool = false; // for {2}, {3,4}

    let mut character_start_option: Option<char> = None;
    let mut character_end_option: Option<char> = None;

    // let mut recombine: u8 = 0;
    // let mut current_recombine: u8 = 0;

    let mut left: bool = true;

    println!("Input: {}", regex_infix);

    let mut root_index: usize = 0;
    let mut root_stack:Vec<NodeId> = Vec::new();
    root_stack.push(NodeId { index: 0 });
    root_stack.push(NodeId { index: 0 });
    root_stack.push(NodeId { index: 0 });
    root_stack.push(NodeId { index: 0 });
    root_stack.push(NodeId { index: 0 });
    root_stack.push(NodeId { index: 0 });
    root_stack.push(NodeId { index: 0 });

    let mut chars = regex_infix.chars().fuse();
    while let Some(c) = chars.next() {

        // current_recombine = 0;

        root_node_id = root_stack[root_index].clone();

        // DEBUG
        println!("");
        println!("{}", c);
        if !arena.is_empty() {
            println!("root id: {:?}", root_node_id.index);
            for (pos, e) in arena.nodes.iter().enumerate() {
                println!("Node {}, value: {:?}", pos, e.data);
                match &e.left {
                    Some(_) => {
                        println!("    Left {}", e.left.as_ref().unwrap().index);
                    }
                    None => {
                    }
                }
                match &e.right {
                    Some(_) => {
                        println!("    Right {}", e.right.as_ref().unwrap().index);
                    }
                    None => {
                    }
                }
            }
        }

        match c {

            '!' => { panic!(); }

            // # not allowed in regex format
            '#' => { panic!(); }

            '+' | '*' | '?' => {
                let mut min:u8 = 0;
                let mut max:u8 = std::u8::MAX;
                if c == '+' {
                    min = 1;
                }
                if c == '?' {
                    min = 0;
                    max = 1;
                }
                let mut repeat = RegexBuildingBlock::Repeat(min, max);

                if arena.is_empty() {
                    panic!("illegal");
                } else {

                    let mut inserted: bool = false;
                    let mut update_root_node_id: bool = true;

                    let mut last_node_id: NodeId = root_node_id.clone();
                    let mut node_id: NodeId = root_node_id.clone();

                    while !inserted {

                        let root_value = arena.get_payload(&node_id);
                        match root_value {

                            RegexBuildingBlock::Concatenation => {

                                node_id = descend_into_concatenation_right_side(&arena, &node_id).clone();

                                // let mut bottom_reached: bool = false;
                                // while !bottom_reached {
                                //     // check if right node is #
                                //     match arena.get_payload(&arena.get_right_id(&node_id).unwrap()) {
                                //         RegexBuildingBlock::Concatenation => {
                                //             node_id = arena.get_right_id(&node_id).unwrap().clone();
                                //         }
                                //         _ => { bottom_reached = true; }
                                //     }
                                // }

                                // repeat is applied at the root of the tree
                                if node_id.index == root_node_id.index {

                                    arena.change_payload(&node_id, repeat);

                                    let concatenation_node_id: NodeId = arena.new_node(RegexBuildingBlock::Concatenation);

                                    // insert old node into the left side of new node
                                    arena.insert_left(&concatenation_node_id, node_id.clone());
                                    //arena.insert_right(&concatenation_node_id, node_id.clone());

                                    // // insert new literal into the right side
                                    // arena.add_right(&new_root_node_id, repeat);

                                    // new node becomes root node
                                    if update_root_node_id {
                                        root_node_id.index = concatenation_node_id.index;
                                        root_stack[root_index].index = concatenation_node_id.index;
                                    }

                                    inserted = true;

                                } else {

                                    // repeat is applied in the middle of the tree
                                    arena.insert_repeat_node_into_node(&node_id, repeat.clone());

                                    inserted = true;
                                }
                            }

                            RegexBuildingBlock::OpeningBraces => {
                                // CHANGE
                                //let node_id_option = arena.get_left_id(&node_id);
                                let node_id_option = arena.get_right_id(&node_id);

                                match node_id_option {
                                    Some(right_child_node_id) => {
                                        last_node_id = node_id.clone();
                                        node_id = right_child_node_id.clone();

                                        update_root_node_id = false;
                                        inserted = false;
                                    }
                                    None => {
                                        panic!("test");
                                    }
                                }
                                
                            }

                            RegexBuildingBlock::ClosedBraces | RegexBuildingBlock::CharacterLiteral(_) | RegexBuildingBlock::CharacterClass(_, _) => {

                                // CHANGE_2 -- needed for (d+c*) but breaks all other tests
                                // e.g. breaks test 4:  a+

                                // repeat is applied at the root of the tree
                                if node_id.index == root_node_id.index {

                                    // arena.change_payload(&node_id, repeat);

                                    // let concatenation_node_id: NodeId = arena.new_node(repeat.clone());

                                    // // insert old node into the left side of new node
                                    // arena.insert_left(&concatenation_node_id, node_id.clone());
                                    // //arena.insert_right(&concatenation_node_id, node_id.clone());

                                    // // // insert new literal into the right side
                                    // // arena.add_right(&new_root_node_id, repeat);

                                    // // new node becomes root node
                                    // if update_root_node_id {
                                    //     root_node_id.index = concatenation_node_id.index;
                                    //     root_stack[root_index].index = concatenation_node_id.index;
                                    // }

                                    let new_root_node_id: NodeId = arena.new_node(repeat);

                                    // insert old node into the left side of new node
                                    arena.insert_left(&new_root_node_id, node_id.clone());

                                    // new node becomes root node
                                    if update_root_node_id {
                                        root_node_id.index = new_root_node_id.index;
                                        root_stack[root_index].index = new_root_node_id.index;
                                    }

                                    inserted = true;

                                } else {
                                    // repeat is applied in the middle of the tree
                                    arena.insert_repeat_node_into_node(&last_node_id, repeat.clone());

                                    inserted = true;
                                }

                            }

                            RegexBuildingBlock::Or => {
                                let node_id_option = arena.get_right_id(&node_id);
                                match node_id_option {

                                    Some(right_child_node_id) => {
                                        
                                        // CHANGE
                                        //left = true;
                                        left = false;

                                        last_node_id = node_id.clone();

                                        // CHANGE
                                        //node_id = left_child_node_id.clone();
                                        node_id = right_child_node_id.clone();

                                        update_root_node_id = false;
                                        inserted = false;
                                    }
                                    None => {
                                        panic!("error");
                                    }
                                }
                            }

                            _ => { panic!("NYI"); }
                        }
                    }
                }
            }
            '{' => {
                if character_class_mode || repeat_mode {
                    panic!("Illegal Syntax! Nested brackets!");
                }
                repeat_mode = true;
            }
            '}' => {
                if !repeat_mode {
                    panic!("Illegal Syntax! '}}' used without opening brackets!");
                }

                if character_end_option == None {
                    character_end_option = character_start_option;
                }

                let mut repeat = RegexBuildingBlock::Repeat(
                    (character_start_option.unwrap() as u8 - 0x30) as u8, 
                    (character_end_option.unwrap() as u8 - 0x30) as u8
                );

                if arena.is_empty() {
                    root_node_id = arena.new_node(repeat);
                } else {
                    let root_value = arena.get_payload(&root_node_id);
                    match root_value {

                        RegexBuildingBlock::Concatenation => {

                            arena.change_payload(&root_node_id, repeat);

                            let new_root_node_id: NodeId = arena.new_node(RegexBuildingBlock::Concatenation);

                            // insert old node into the left side of new node
                            arena.insert_left(&new_root_node_id, root_node_id.clone());

                            // // insert new literal into the right side
                            // arena.add_right(&new_root_node_id, repeat);

                            // new node becomes root node
                            root_node_id.index = new_root_node_id.index;
                            root_stack[root_index].index = new_root_node_id.index;
                        }

                        RegexBuildingBlock::CharacterLiteral(_) | RegexBuildingBlock::CharacterClass(_, _) => {

                            let new_root_node_id: NodeId = arena.new_node(repeat);
                            // insert old node into the left side of new node
                            arena.insert_left(&new_root_node_id, root_node_id.clone());

                            // new node becomes root node
                            root_node_id.index = new_root_node_id.index;
                            root_stack[root_index].index = new_root_node_id.index;

                        }

                        _ => { panic!("NIY"); }
                    }
                }

                character_start_option = None;
                character_end_option = None;
                repeat_mode = false;
            }

            //
            // Character Class
            //

            '-' => {}
            '[' => { 
                if character_class_mode || repeat_mode {
                    panic!("Illegal Syntax! Nested brackets!");
                }
                character_class_mode = true;
             }
            ']' => { 
                if !character_class_mode {
                    panic!("Illegal Syntax! ']' used without opening brackets!");
                }

                let mut character_class = RegexBuildingBlock::CharacterClass(character_start_option.unwrap(), character_end_option.unwrap());
                if arena.is_empty() {
                    root_node_id = arena.new_node(character_class);
                } else {
                    let root_value = arena.get_payload(&root_node_id);
                    match root_value {

                        RegexBuildingBlock::CharacterLiteral(_) | RegexBuildingBlock::CharacterClass(_, _) | RegexBuildingBlock::Concatenation => {

                            let new_root_node_id: NodeId = arena.new_node(RegexBuildingBlock::Concatenation);
                            // insert old node into the left side of new node
                            arena.insert_left(&new_root_node_id, root_node_id.clone());

                            // insert new literal into the right side
                            arena.add_right(&new_root_node_id, character_class);

                            // new node becomes root node
                            root_node_id.index = new_root_node_id.index;
                            root_stack[root_index].index = new_root_node_id.index;
                        }

                        RegexBuildingBlock::Repeat(min, max) => {

                            let new_root_node_id: NodeId = arena.new_node(RegexBuildingBlock::Concatenation);
                            // insert old node into the left side of new node
                            arena.insert_left(&new_root_node_id, root_node_id.clone());

                            // insert new literal into the right side
                            arena.add_right(&new_root_node_id, character_class);

                            // new node becomes root node
                            root_node_id.index = new_root_node_id.index;
                            root_stack[root_index].index = new_root_node_id.index;
                        }

                        _ => { panic!("NIY"); }
                    }
                }

                character_start_option = None;
                character_end_option = None;
                character_class_mode = false;
            }

            //
            // braces
            //

            '(' => {

                if arena.is_empty() {
                    root_node_id = arena.new_node(RegexBuildingBlock::OpeningBraces);

                    //root_index = root_index + 1;
                    root_stack[root_index] = root_node_id.clone();

                } else {

                    let mut inserted: bool = false;
                    let mut update_root_node_id: bool = true;

                    let mut last_node_id: NodeId = root_node_id.clone();
                    let mut node_id: NodeId = root_node_id.clone();

                    while !inserted {

                        let root_value = arena.get_payload(&node_id);
                        match root_value {

                            RegexBuildingBlock::OpeningBraces => {
                                // CHANGE
                                //let node_id_option = arena.get_left_id(&node_id);
                                let node_id_option = arena.get_right_id(&node_id);

                                match node_id_option {
                                    Some(left_child_node_id) => {
                                        last_node_id = node_id.clone();
                                        node_id = left_child_node_id.clone();

                                        update_root_node_id = false;
                                        inserted = false;
                                    }
                                    None => {
                                        // CHANGE
                                        //arena.add_left(&node_id, RegexBuildingBlock::OpeningBraces);
                                        arena.add_right(&node_id, RegexBuildingBlock::OpeningBraces);

                                        inserted = true;
                                    }
                                }
                                
                            }

                            RegexBuildingBlock::ClosedBraces => {

                                let id = concat_right_side(&mut arena, &last_node_id, RegexBuildingBlock::OpeningBraces);

                                // let new_root_node_id: NodeId = arena.new_node(RegexBuildingBlock::Concatenation);
                                // // insert old node into the left side of new node
                                // arena.insert_left(&new_root_node_id, root_node_id.clone());

                                // // insert new literal into the right side
                                // let right_node_id = arena.add_right(&new_root_node_id, RegexBuildingBlock::OpeningBraces);

                                // // new node becomes root node
                                // if update_root_node_id {
                                //     root_node_id.index = new_root_node_id.index;
                                //     root_stack[root_index].index = new_root_node_id.index;
                                // }

                                root_index = root_index + 1;
                                root_stack[root_index].index = id;

                                inserted = true;
                            }

                            RegexBuildingBlock::CharacterLiteral(c) => {

                                let concat_node_id: NodeId = arena.new_node(RegexBuildingBlock::Concatenation);
                                // insert old node into the left side of new node
                                arena.insert_left(&concat_node_id, node_id.clone());

                                // insert new literal into the right side
                                let right_node_id = arena.add_right(&concat_node_id, RegexBuildingBlock::OpeningBraces);

                                // new node becomes root node
                                if update_root_node_id {
                                    root_node_id.index = concat_node_id.index;
                                    root_stack[root_index].index = concat_node_id.index;
                                } else {
                                    arena.insert_right(&last_node_id, concat_node_id.clone());
                                }

                                root_index = root_index + 1;
                                root_stack[root_index] = right_node_id.clone();

                                inserted = true;
                            }

                            RegexBuildingBlock::Or => {
                                let node_id_option = arena.get_right_id(&node_id);
                                match node_id_option {

                                    Some(right_child_node_id) => {
                                        
                                        // CHANGE
                                        //left = true;
                                        left = false;

                                        last_node_id = node_id.clone();

                                        // CHANGE
                                        //node_id = left_child_node_id.clone();
                                        node_id = right_child_node_id.clone();

                                        update_root_node_id = false;
                                        inserted = false;
                                    }
                                    None => {
                                        // insert new literal into the right side
                                        let child_node_id = arena.add_right(&node_id, RegexBuildingBlock::OpeningBraces);

                                        root_index = root_index + 1;
                                        root_stack[root_index] = child_node_id.clone();

                                        inserted = true;
                                    }
                                }
                            }

                            _ => {

                                // CHANGE
                                // let new_root_node_id: NodeId = arena.new_node(RegexBuildingBlock::OpeningBraces);
                                // // insert old node into the left side of new node
                                // arena.insert_left(&new_root_node_id, root_node_id.clone());
                                // // new node becomes root node
                                // root_node_id.index = new_root_node_id.index;

                                let concat_node_id: NodeId = arena.new_node(RegexBuildingBlock::Concatenation);
                                // insert old node into the left side of new node
                                arena.insert_left(&concat_node_id, node_id.clone());

                                // insert new literal into the right side
                                let opening_braces_id = arena.add_right(&concat_node_id, RegexBuildingBlock::OpeningBraces);

                                // new node becomes root node
                                if update_root_node_id {
                                    root_node_id.index = concat_node_id.index;
                                    root_stack[root_index].index = concat_node_id.index;
                                } else {
                                    arena.insert_right(&last_node_id, concat_node_id.clone());
                                }

                                inserted = true;

                                root_index = root_index + 1;
                                root_stack[root_index] = opening_braces_id.clone();
                            }
                        }
                    }
                }
            }

            ')' => {
                if arena.is_empty() {
                    panic!("invalid!");
                } else {

                    let mut inserted: bool = false;
                    let mut update_root_node_id: bool = true;

                    let mut last_node_id: NodeId = root_node_id.clone();
                    let mut node_id: NodeId = root_node_id.clone();

                    while !inserted {

                        let root_value = arena.get_payload(&node_id);
                        match root_value {

                            RegexBuildingBlock::Concatenation => {

                                let node_id_option = arena.get_right_id(&node_id);
                                match node_id_option {
                                    Some(right_child_node_id) => {
                                        last_node_id = node_id.clone();
                                        node_id = right_child_node_id.clone();

                                        update_root_node_id = false;
                                        inserted = false;
                                    }
                                    None => {
                                        panic!("test");
                                    }
                                }
                            }

                            RegexBuildingBlock::OpeningBraces => {

                                arena.change_payload(&node_id, RegexBuildingBlock::ClosedBraces);
                                inserted = true;

                                if root_index != 0 {
                                    root_index = root_index - 1;
                                }

                                // current_recombine = current_recombine + 1;
                                // if recombine == current_recombine {
                                //     arena.change_payload(&node_id, RegexBuildingBlock::ClosedBraces);

                                //     recombine = recombine - 1;
                                //     inserted = true;
                                // } else {

                                //     // CHANGE
                                //     //let node_id_option = arena.get_left_id(&node_id);
                                //     let node_id_option = arena.get_right_id(&node_id);

                                //     match node_id_option {
                                //         Some(right_child_node_id) => {
                                //             last_node_id = node_id.clone();
                                //             node_id = right_child_node_id.clone();

                                //             update_root_node_id = false;
                                //             inserted = false;
                                //             //panic!("tset");
                                //         }
                                //         None => {
                                //             panic!("test");
                                //         }
                                //     }
                                // }
                            }

                            RegexBuildingBlock::Or => {
                                let node_id_option = arena.get_right_id(&node_id);
                                match node_id_option {
                                    Some(right_child_node_id) => {
                                        last_node_id = node_id.clone();
                                        node_id = right_child_node_id.clone();

                                        update_root_node_id = false;
                                        inserted = false;
                                    }
                                    None => {
                                        panic!("test");
                                    }
                                }
                            }

                            _ => { 
                                panic!("invalid"); 
                            }
                        }
                    }
                }
            }

            '|' => {

                let mut inserted: bool = false;
                let mut update_root_node_id: bool = true;

                let mut last_node_id: NodeId = root_node_id.clone();
                let mut node_id: NodeId = root_node_id.clone();

                while !inserted {

                    let root_value = arena.get_payload(&node_id);
                    match root_value {

                        RegexBuildingBlock::Repeat(min, max) => {

                            let new_root_node_id: NodeId = arena.new_node(RegexBuildingBlock::Or);
                            // insert old node into the left side of new node
                            arena.insert_left(&new_root_node_id, root_node_id.clone());

                            // insert new literal into the right side
                            //arena.add_right(&new_root_node_id, character_class);

                            // new node becomes root node
                            root_node_id.index = new_root_node_id.index;
                            root_stack[root_index].index = new_root_node_id.index;

                            inserted = true;
                        }

                        RegexBuildingBlock::CharacterLiteral(_) | RegexBuildingBlock::CharacterClass(_, _) | RegexBuildingBlock::Concatenation => {

                            // OR is applied at the root of the tree
                            if node_id.index == root_node_id.index {

                                let new_root_node_id: NodeId = arena.new_node(RegexBuildingBlock::Or);

                                // insert old node into the left side of new node
                                arena.insert_left(&new_root_node_id, node_id.clone());

                                // new node becomes root node
                                //if update_root_node_id {
                                    root_node_id.index = new_root_node_id.index;
                                    root_stack[root_index].index = new_root_node_id.index;
                                //}

                                inserted = true;
                            } else {
                                panic!("test");
                            }
                        }

                        RegexBuildingBlock::Or => {

                            // OR is applied at the root of the tree
                            if node_id.index == root_node_id.index {

                                let new_root_node_id: NodeId = arena.new_node(RegexBuildingBlock::Or);

                                // insert old node into the left side of new node
                                arena.insert_left(&new_root_node_id, node_id.clone());

                                // new node becomes root node
                                //if update_root_node_id {
                                    root_node_id.index = new_root_node_id.index;
                                    root_stack[root_index].index = new_root_node_id.index;
                                //}

                                inserted = true;
                            } else {
                                panic!("test");
                            }
                        }

                        RegexBuildingBlock::ClosedBraces => {
                            // OR is applied at the root of the tree
                            if node_id.index == root_node_id.index {

                                let new_root_node_id: NodeId = arena.new_node(RegexBuildingBlock::Or);

                                // insert old node into the left side of new node
                                arena.insert_left(&new_root_node_id, node_id.clone());

                                // new node becomes root node
                                //if update_root_node_id {
                                    root_node_id.index = new_root_node_id.index;
                                    root_stack[root_index].index = new_root_node_id.index;
                                //}

                                inserted = true;
                            } else {
                                panic!("test");
                            }
                        }

                        _ => {
                            panic!("test");
                        }
                    }
                }
            }

            _ => {

                // if in bracket_mode build up the character class operator with start and end node
                if character_class_mode { // e.g. [a-z]
                    if !c.is_alphanumeric() {
                        panic!("Range border in repeat operator is not a numeric character!");
                    }
                    if character_start_option.is_none() {
                        character_start_option = Some(c);
                    } else {
                        character_end_option = Some(c);
                    }
                } else if repeat_mode { // e.g {3}, {2,4}
                    if !c.is_numeric() && c != ',' {
                        panic!("Range border in repeat operator is not a numeric character!");
                    }
                    if character_start_option.is_none() {
                        character_start_option = Some(c);
                        character_end_option = Some(c);
                    } else {
                        character_end_option = Some(c);
                    }
                } else {
                    let mut character_literal = RegexBuildingBlock::CharacterLiteral(c);
                    if arena.is_empty() {
                        root_node_id = arena.new_node(character_literal);
                    } else {

                        let mut inserted: bool = false;
                        let mut update_root_node_id: bool = true;

                        let mut last_node_id: NodeId = root_node_id.clone();
                        let mut node_id: NodeId = root_node_id.clone();

                        while !inserted {
                        
                            let root_value = arena.get_payload(&node_id);
                            match root_value {

                                RegexBuildingBlock::OpeningBraces => {
                                    
                                    // CHANGE
                                    //let node_id_option = arena.get_left_id(&node_id);
                                    let node_id_option = arena.get_right_id(&node_id);

                                    match node_id_option {
                                        // CHANGE
                                        //Some(left_child_node_id) => {
                                        Some(right_child_node_id) => {
                                            
                                            // CHANGE
                                            //left = true;
                                            left = false;

                                            last_node_id = node_id.clone();

                                            // CHANGE
                                            //node_id = left_child_node_id.clone();
                                            node_id = right_child_node_id.clone();

                                            update_root_node_id = false;
                                            inserted = false;
                                        }
                                        None => {
                                            // CHANGE
                                            //arena.add_left(&node_id, character_literal);
                                            arena.add_right(&node_id, character_literal);

                                            inserted = true;
                                        }
                                    }
                                }

                                RegexBuildingBlock::ClosingBraces => {
                                    //println!("test");
                                    //panic!("ClosingBraces");
                                }

                                RegexBuildingBlock::ClosedBraces => {
                                    println!("ClosedBraces");

                                    let concat_node_id: NodeId = arena.new_node(RegexBuildingBlock::Concatenation);
                                    
                                    // insert old node into the left side of new node
                                    arena.insert_left(&concat_node_id, node_id.clone());
                                    //arena.insert_right(&concat_node_id, node_id.clone());

                                    // insert new literal into the right side
                                    arena.add_right(&concat_node_id, character_literal);

                                    // new node becomes root node
                                    if update_root_node_id {
                                        root_node_id.index = concat_node_id.index;
                                        root_stack[root_index].index = concat_node_id.index;
                                    } else {
                                        //arena.insert_left(&last_node_id, concatenation_node_id.clone());
                                        arena.insert_right(&last_node_id, concat_node_id.clone());
                                    }

                                    inserted = true;
                                }

                                RegexBuildingBlock::Concatenation => {
                                    let node_id_option = arena.get_right_id(&node_id);

                                    // // CHANGE
                                    // match node_id_option {
                                    //     Some(left_child_node_id) => {
                                    //         left = false;

                                    //         last_node_id = node_id.clone();
                                    //         node_id = left_child_node_id.clone();

                                    //         update_root_node_id = false;
                                    //         inserted = false;
                                    //     }
                                    //     None => {
                                    //         let new_root_node_id: NodeId = arena.new_node(RegexBuildingBlock::Concatenation);
                                    //         // insert old node into the left side of new node
                                    //         arena.insert_left(&new_root_node_id, node_id.clone());

                                    //         // insert new literal into the right side
                                    //         arena.add_right(&new_root_node_id, character_literal);

                                    //         // new node becomes root node
                                    //         if update_root_node_id {
                                    //             root_node_id.index = new_root_node_id.index;
                                    //         } else {
                                    //             arena.insert_left(&last_node_id, new_root_node_id.clone());
                                    //         }

                                    //         inserted = true;
                                    //     }
                                    // }

                                    let new_root_node_id: NodeId = arena.new_node(RegexBuildingBlock::Concatenation);
                                    // insert old node into the left side of new node
                                    arena.insert_left(&new_root_node_id, node_id.clone());

                                    // insert new literal into the right side
                                    arena.add_right(&new_root_node_id, character_literal);

                                    // new node becomes root node
                                    if update_root_node_id {
                                        root_node_id.index = new_root_node_id.index;
                                        root_stack[root_index].index = new_root_node_id.index;
                                    } else {
                                        arena.insert_right(&last_node_id, new_root_node_id.clone());
                                    }

                                    inserted = true;
                                }

                                RegexBuildingBlock::CharacterLiteral(_) | RegexBuildingBlock::CharacterClass(_, _) => {

                                    let concatenation_node_id: NodeId = arena.new_node(RegexBuildingBlock::Concatenation);
                                    // insert old node into the left side of new node
                                    arena.insert_left(&concatenation_node_id, node_id.clone());

                                    // insert new literal into the right side
                                    arena.add_right(&concatenation_node_id, character_literal);
                                    
                                    if update_root_node_id {
                                        // new node becomes root node
                                        root_node_id.index = concatenation_node_id.index;
                                        root_stack[root_index].index = concatenation_node_id.index;
                                    } else {
                                        if left {
                                             arena.insert_left(&last_node_id, concatenation_node_id.clone());
                                        } else {
                                            arena.insert_right(&last_node_id, concatenation_node_id.clone());
                                        }
                                    }

                                    inserted = true;
                                }

                                RegexBuildingBlock::Repeat(min, max) => {

                                    let new_root_node_id: NodeId = arena.new_node(RegexBuildingBlock::Concatenation);
                                    // insert old node into the left side of new node
                                    arena.insert_left(&new_root_node_id, node_id.clone());
        
                                    // insert new literal into the right side
                                    arena.add_right(&new_root_node_id, character_literal);

                                    if node_id.index == root_node_id.index {
                                        // new node becomes root node
                                        //if update_root_node_id {
                                            root_node_id.index = new_root_node_id.index;
                                            root_stack[root_index].index = new_root_node_id.index;
                                        //}
                                    } else {
                                        arena.insert_right(&last_node_id, new_root_node_id.clone());
                                    }

                                    inserted = true;
                                }

                                RegexBuildingBlock::Or => {

                                    let node_id_option = arena.get_right_id(&node_id);
                                    match node_id_option {

                                        Some(right_child_node_id) => {
                                            
                                            // CHANGE
                                            //left = true;
                                            left = false;

                                            last_node_id = node_id.clone();

                                            // CHANGE
                                            //node_id = left_child_node_id.clone();
                                            node_id = right_child_node_id.clone();

                                            update_root_node_id = false;
                                            inserted = false;
                                        }
                                        None => {
                                            // insert new literal into the right side
                                            arena.add_right(&node_id, character_literal);

                                            inserted = true;
                                        }
                                    }
                                }

                                _ => { panic!("NIY"); }
                            }
                        }
                    }
                }
            }
        }
    }

    //recurse(&arena, &root_node_id);
    //println!("");

    let mut string_buffer = String::from("");

    root_node_id = root_stack[root_index].clone();

    recurse_postfix(&arena, &root_node_id, &mut string_buffer);
    println!("");

    string_buffer
}

pub fn main() {

    //let regex_infix: &str = "a|b";                // 1
    //let regex_infix: &str = "a|b|c";              // 2
    //let regex_infix: &str = "ab|cd";              // 3
    //let regex_infix: &str = "x|ab";               // 4
    //let regex_infix: &str = "x|(a)";              // 5
    //let regex_infix: &str = "x|(a)(b)";           // 6
    //let regex_infix: &str = "(ab)|(cd)";          // 7
    //let regex_infix: &str = "(ab)+|(cd)";         // 8
    let regex_infix: &str = "(ab)|(cd)+";         // 9
    //let regex_infix: &str = "(ab)+|(cd)+";        // 10
    //let regex_infix: &str = "(ab)+|(c?d)+";       // 11

    //let regex_infix: &str = "a";                  // 1
    //let regex_infix: &str = "ab";                 // 2
    //let regex_infix: &str = "abc";                 // 3
    //let regex_infix: &str = "a+";                 // 4
    //let regex_infix: &str = "a*";                 // 5
    //let regex_infix: &str = "[a-z]";              // 6
    //let regex_infix: &str = "[a-z]x";              // 7
    //let regex_infix: &str = "x[a-z]";              // 8
    //let regex_infix: &str = "[a-z]x[a-z]";              // 9
    //let regex_infix: &str = "[a-z][0-9]";              // 4
    //let regex_infix: &str = "[a-z]+";              // 4
    //let regex_infix: &str = "[a-z]*";              // 4
    //let regex_infix: &str = "[a-z]?";              // 4
    //let regex_infix: &str = "[a-z]{3}";              // 4
    //let regex_infix: &str = "[a-z]{3,4}";              // 4
    //let regex_infix: &str = "[a-z]+[0-9]";              // 4
    //let regex_infix: &str = "[a-z]*[0-9]";              // 4
    //let regex_infix: &str = "[a-z]?[0-9]";              // 4
    //let regex_infix: &str = "[a-z]{3}[0-9]";              // 4
    //let regex_infix: &str = "[a-z]{3,4}[0-9]";              // 4
    //let regex_infix: &str = "a{3}";               // 5
    //let regex_infix: &str = "a{3,4}";             // 6
    //let regex_infix: &str = "[a-z][0-9]";         // 7
    //let regex_infix: &str = "[a-z]+[0-9]+";       // 8
    //let regex_infix: &str = "[a-z]{3,4}[0-9]{3,4}";       // 8
    //let regex_infix: &str = "[a-z][0-9]{2,3}";    // 9
    //let regex_infix: &str = "ab+";                // 13
    //let regex_infix: &str = "b[a-c]bble";         // 14 broken
    //let regex_infix: &str = "[a-z][0-9]{2,3}+";   // 15
    //let regex_infix: &str = "[a-z]+[0-9]";        // 16
    //let regex_infix: &str = "a?";                 // 17
    //let regex_infix: &str = "abc+";               // 18 broken
    //let regex_infix: &str = "abc?";               // 18 broken
    //let regex_infix: &str = "d+c*";               // 19
    //let regex_infix: &str = "(a)";                // 20
    //let regex_infix: &str = "(ab)";                // 20
    //let regex_infix: &str = "a(b)";                // 20
    //let regex_infix: &str = "(a)(b)";             // 21
    //let regex_infix: &str = "(a(b))";             // 21
    //let regex_infix: &str = "(a(b)c)";             // 21
    //let regex_infix: &str = "(a(b))";             // 24 
    //let regex_infix: &str = "(a)+";               // 26
    //let regex_infix: &str = "(ab)+";              // 27
    //let regex_infix: &str = "(a(b))+";            // 25
    //let regex_infix: &str = "(a(b)+)+";            // 25 // ab+#+
    //let regex_infix: &str = "(ab(cd)+ef)";        // 28
    //let regex_infix: &str = "(ab(cd)+ef)+";       // 29
   
    //let regex_infix: &str = "(d+c*)";             // 36
    //let regex_infix: &str = "c(d+c*)";            // 37
    //let regex_infix: &str = "c(d+c*)bc";          // 38

    infix_to_postfix(regex_infix);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // a|b
    // a|b|c
    // x|ab
    // x|(a)
    // x|(a)(b)
    // ab|cd
    // (ab)|(cd)
    // (ab)+|(cd)
    // (ab)|(cd)+
    // (ab)+|(cd)+
    // (ab)+|(c?d)+

    #[test]
    fn regex_or_1_test() {
        assert_eq!("ab|", infix_to_postfix("a|b"));
    }

    #[test]
    fn regex_or_2_test() {
        assert_eq!("ab|c|", infix_to_postfix("a|b|c"));
    }

    #[test]
    fn regex_or_3_test() {
        assert_eq!("xab#|", infix_to_postfix("x|ab"));
    }

    #[test]
    fn regex_or_4_test() {
        assert_eq!("xa|", infix_to_postfix("x|(a)"));
    }

    #[test]
    fn regex_or_5_test() {
        assert_eq!("xab#|", infix_to_postfix("x|(a)(b)"));
    }

    #[test]
    fn regex_or_6_test() {
        assert_eq!("ab#cd#|", infix_to_postfix("ab|cd"));
    }

    #[test]
    fn regex_or_7_test() {
        assert_eq!("ab#cd#|", infix_to_postfix("(ab)|(cd)"));
    }

    #[test]
    fn regex_or_8_test() {
        assert_eq!("ab#+cd#|", infix_to_postfix("(ab)+|(cd)"));
    }

    #[test]
    fn regex_or_9_test() {
        assert_eq!("ab#cd#+|", infix_to_postfix("(ab)|(cd)+"));
    }

    #[test]
    fn regex_or_10_test() {
        assert_eq!("ab#+cd#+|", infix_to_postfix("(ab)+|(cd)+"));
    }

    #[test]
    fn regex_or_11_test() {
        assert_eq!("ab#+c?d#+|", infix_to_postfix("(ab)+|(c?d)+"));
    }



    #[test]
    fn regex_1_test() {
        assert_eq!("a", infix_to_postfix("a"));
    }

    #[test]
    fn regex_2_test() {
        assert_eq!("ab#", infix_to_postfix("ab"));
    }

    #[test]
    fn regex_3_test() {
        assert_eq!("ab#c#", infix_to_postfix("abc"));
    }

    #[test]
    fn regex_4_test() {
        assert_eq!("a+", infix_to_postfix("a+"));
    }

    #[test]
    fn regex_5_test() {
        assert_eq!("a*", infix_to_postfix("a*"));
    }

    #[test]
    fn regex_6_test() {
        assert_eq!("[a-z]", infix_to_postfix("[a-z]"));
    }

    #[test]
    fn regex_7_test() {
        assert_eq!("[a-z]x#", infix_to_postfix("[a-z]x"));
    }

    #[test]
    fn regex_8_test() {
        assert_eq!("x[a-z]#", infix_to_postfix("x[a-z]"));
    }

    #[test]
    fn regex_9_test() {
        assert_eq!("[a-z]x#[a-z]#", infix_to_postfix("[a-z]x[a-z]"));
    }

    #[test]
    fn regex_10_test() {
        assert_eq!("[a-z][0-9]#", infix_to_postfix("[a-z][0-9]"));
    }

    #[test]
    fn regex_11_test() {
        assert_eq!("[a-z]+", infix_to_postfix("[a-z]+"));
    }

    #[test]
    fn regex_12_test() {
        assert_eq!("[a-z]*", infix_to_postfix("[a-z]*"));
    }

    #[test]
    fn regex_13_test() {
        assert_eq!("[a-z]?", infix_to_postfix("[a-z]?"));
    }

    #[test]
    fn regex_14_test() {
        assert_eq!("[a-z]{3}", infix_to_postfix("[a-z]{3}"));
    }

    #[test]
    fn regex_15_test() {
        assert_eq!("[a-z]{3,4}", infix_to_postfix("[a-z]{3,4}"));
    }

    #[test]
    fn regex_16_test() {
        assert_eq!("[a-z]+[0-9]#", infix_to_postfix("[a-z]+[0-9]"));
    }

    #[test]
    fn regex_17_test() {
        assert_eq!("[a-z]*[0-9]#", infix_to_postfix("[a-z]*[0-9]"));
    }

    #[test]
    fn regex_18_test() {
        assert_eq!("[a-z]?[0-9]#", infix_to_postfix("[a-z]?[0-9]"));
    }

    #[test]
    fn regex_19_test() {
        assert_eq!("[a-z]{3}[0-9]#", infix_to_postfix("[a-z]{3}[0-9]"));
    }

    #[test]
    fn regex_20_test() {
        assert_eq!("[a-z]{3,4}[0-9]#", infix_to_postfix("[a-z]{3,4}[0-9]"));
    }

    #[test]
    fn regex_21_test() {
        assert_eq!("a{3}", infix_to_postfix("a{3}"));
    }

    #[test]
    fn regex_22_test() {
        assert_eq!("a{3,4}", infix_to_postfix("a{3,4}"));
    }

    #[test]
    fn regex_23_test() {
        assert_eq!("[a-z][0-9]#", infix_to_postfix("[a-z][0-9]"));
    }

    #[test]
    fn regex_24_test() {
        assert_eq!("a{3,4}", infix_to_postfix("a{3,4}"));
    }

    #[test]
    fn regex_25_test() {
        assert_eq!("[a-z][0-9]#", infix_to_postfix("[a-z][0-9]"));
    }

    #[test]
    fn regex_26_test() {
        assert_eq!("[a-z]+[0-9]+#", infix_to_postfix("[a-z]+[0-9]+"));
    }

    #[test]
    fn regex_27_test() {
        assert_eq!("[a-z]{3,4}[0-9]{3,4}#", infix_to_postfix("[a-z]{3,4}[0-9]{3,4}"));
    }

    #[test]
    fn regex_28_test() {
        assert_eq!("[a-z][0-9]{2,3}#", infix_to_postfix("[a-z][0-9]{2,3}"));
    }

    #[test]
    fn regex_29_test() {
        assert_eq!("ab+#", infix_to_postfix("ab+"));
    }

    #[test]
    fn regex_30_test() {
        assert_eq!("b[a-c]#b#b#l#e#", infix_to_postfix("b[a-c]bble"));
    }

    #[test]
    fn regex_31_test() {
        assert_eq!("[a-z][0-9]{2,3}+#", infix_to_postfix("[a-z][0-9]{2,3}+"));
    }

    #[test]
    fn regex_32_test() {
        assert_eq!("[a-z]+[0-9]#", infix_to_postfix("[a-z]+[0-9]"));
    }

    #[test]
    fn regex_33_test() {
        assert_eq!("a?", infix_to_postfix("a?"));
    }

    #[test]
    fn regex_34_test() {
        assert_eq!("ab#c+#", infix_to_postfix("abc+"));
    }

    #[test]
    fn regex_35_test() {
        assert_eq!("ab#c?#", infix_to_postfix("abc?"));
    }

    #[test]
    fn regex_36_test() {
        assert_eq!("d+c*#", infix_to_postfix("d+c*"));
    }
                
    #[test]
    fn regex_37_test() {
        assert_eq!("a", infix_to_postfix("(a)"));
    }

    #[test]
    fn regex_38_test() {
        assert_eq!("ab#", infix_to_postfix("(ab)"));
    } 

    #[test]
    fn regex_39_test() {
        assert_eq!("ab#", infix_to_postfix("(a)(b)"));
    }                

    #[test]
    fn regex_42_test() {
        assert_eq!("ab#", infix_to_postfix("(a(b))"));
    }
    
    #[test]
    fn regex_43_test() {
        assert_eq!("ab#c#", infix_to_postfix("(a(b)c)"));
    }  

    #[test]
    fn regex_46_test() {
        assert_eq!("a+", infix_to_postfix("(a)+"));
    }  

    #[test]
    fn regex_47_test() {
        assert_eq!("ab#+", infix_to_postfix("(ab)+"));
    }
    
    #[test]
    fn regex_48_test() {
        assert_eq!("ab#+", infix_to_postfix("(a(b))+"));
    }

    #[test]
    fn regex_49_test() {
        assert_eq!("ab+#+", infix_to_postfix("(a(b)+)+"));
    }

    #[test]
    fn regex_50_test() {
        assert_eq!("ab#cd#+#e#f#", infix_to_postfix("(ab(cd)+ef)"));
    }

    #[test]
    fn regex_51_test() {
        assert_eq!("ab#cd#+#e#f#+", infix_to_postfix("(ab(cd)+ef)+"));
    }

    #[test]
    fn regex_58_test() {
        assert_eq!("ab#", infix_to_postfix("a(b)"));
    }

    #[test]
    fn regex_60_test() {
        assert_eq!("d+c*#", infix_to_postfix("(d+c*)"));
    }

    #[test]
    fn regex_61_test() {
        assert_eq!("cd+c*##", infix_to_postfix("c(d+c*)"));
    }

    #[test]
    fn regex_62_test() {
        assert_eq!("ab#", infix_to_postfix("a(b)"));
    }

    #[test]
    fn regex_63_test() {
        assert_eq!("cd+c*##b#c#", infix_to_postfix("c(d+c*)bc"));
    }   

}