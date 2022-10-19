use std::fmt::Debug;

struct Node<T: Debug> {
    data: T,
    left: Box<Option<Node<T>>>,
    right: Box<Option<Node<T>>>,
}

impl<T: Debug> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            left: Box::from(None),
            right: Box::from(None),
        }
    }
    fn set_left(&mut self, node: Node<T>) {
        self.left = Box::from(Some(node))
    }
    fn set_right(&mut self, node: Node<T>) {
        self.right = Box::from(Some(node))
    }
}

/// Traverse the left subtree, i.e., call Inorder(left->subtree)
///
/// Visit the root.
///
/// Traverse the right subtree, i.e., call Inorder(right->subtree)
fn inorder<T: Debug>(node: Option<Node<T>>) -> Result<Vec<T>, ()> {
    let mut res: Vec<T> = Vec::new();
    match node {
        Some(value) => {
            match inorder(*value.left) {
                Ok(v) => res.extend(v),
                Err(_) => {}
            }
            res.push(value.data);
            match inorder(*value.right) {
                Ok(v) => res.extend(v),
                Err(_) => {}
            }
            return Ok(res);
        }
        None => return Err(()),
    }
}

// TODO: inorder without recursion

/// Visit the root.
///
/// Traverse the left subtree, i.e., call Preorder(left->subtree)
///
/// Traverse the right subtree, i.e., call Preorder(right->subtree)
fn preorder<T: Debug>(node: Option<Node<T>>) -> Result<Vec<T>, ()> {
    let mut res: Vec<T> = Vec::new();
    match node {
        Some(value) => {
            res.push(value.data);
            match preorder(*value.left) {
                Ok(v) => res.extend(v),
                Err(_) => {}
            }
            match preorder(*value.right) {
                Ok(v) => res.extend(v),
                Err(_) => {}
            }
            return Ok(res);
        }
        None => return Err(()),
    }
}

/// Traverse the left subtree, i.e., call Postorder(left->subtree)
///
/// Traverse the right subtree, i.e., call Postorder(right->subtree)
///
/// Visit the root
fn postorder<T: Debug>(node: Option<Node<T>>) -> Result<Vec<T>, ()> {
    let mut res: Vec<T> = Vec::new();
    match node {
        Some(value) => {
            match postorder(*value.left) {
                Ok(v) => res.extend(v),
                Err(_) => {}
            }
            match postorder(*value.right) {
                Ok(v) => res.extend(v),
                Err(_) => {}
            }
            res.push(value.data);
            return Ok(res);
        }
        None => return Err(()),
    }
}

// DFS

#[derive(Clone)]
struct Graph {
    visited: Vec<bool>,
    adj_lists: Vec<Vec<usize>>,
}

impl Graph {
    fn new(v: usize) -> Self {
        Self {
            visited: vec![false; v],
            adj_lists: vec![Vec::new(); v],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj_lists[src].push(dest);
    }

    fn dfs(&mut self, vertex: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        self.visited[vertex] = true;
        result.push(vertex);
        for v in self.adj_lists[vertex].clone().iter() {
            if !self.visited[*v] {
                let res = self.dfs(*v);
                result.extend(res)
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_graph() -> Node<i32> {
        let mut root = Node::new(1_i32);
        let mut node_1 = Node::new(2_i32);
        let mut node_2 = Node::new(3_i32);
        let mut node_3 = Node::new(4_i32);
        let mut node_4 = Node::new(5_i32);

        node_2.set_left(Node::new(6_i32));
        node_2.set_right(Node::new(7_i32));

        node_3.set_left(Node::new(8_i32));
        node_3.set_right(Node::new(9_i32));

        node_4.set_left(Node::new(10_i32));
        node_4.set_right(Node::new(11_i32));

        node_1.set_left(node_3);
        node_1.set_right(node_4);

        root.set_left(node_1);
        root.set_right(node_2);

        root
    }

    #[test]
    fn test_inorder() {
        let node = init_graph();
        if let Ok(res) = inorder(Some(node)) {
            assert_eq!(vec![8, 4, 9, 2, 10, 5, 11, 1, 6, 3, 7], res);
        };
    }

    #[test]
    fn test_preorder() {
        let node = init_graph();
        if let Ok(res) = preorder(Some(node)) {
            assert_eq!(vec![1, 2, 4, 8, 9, 5, 10, 11, 3, 6, 7], res);
        };
    }

    #[test]
    fn test_postorder() {
        let node = init_graph();
        if let Ok(res) = postorder(Some(node)) {
            assert_eq!(vec![8, 9, 4, 10, 11, 5, 2, 6, 7, 3, 1], res);
        };
    }

    #[test]
    fn test_dfs() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 3);
        graph.add_edge(1, 2);
        graph.add_edge(2, 4);

        let result: Vec<usize> = graph.dfs(0);
        assert_eq!(vec![0, 1, 2, 4, 3], result);
    }
}
