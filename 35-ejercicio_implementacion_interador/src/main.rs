struct Grupos<T> {
    inner: Vec<T>,
}

impl<T> Grupos<T> {
    fn new(inner: Vec<T>) -> Self{
        Grupos { inner } 
    }
}

impl<T: PartialEq> Iterator for Grupos<T> {
    
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.inner.is_empty(){
            return None;
        } 

        let mut grupo = vec![self.inner.remove(0)];
        while !self.inner.is_empty() && self.inner[0] == grupo[0] {
            grupo.push(self.inner.remove(0));
        }
        Some(grupo)
    }    
}

fn main() {
    let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, 5, 5];
    // grupos       -> 
    assert_eq!(
        Grupos::new(data).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![4],
            vec![1, 1],
            vec![2],
            vec![1],
            vec![3, 3],
            vec![-2, -2, -2],
            vec![5, 5],
        ]
    );

    let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
    // groups:      |->|---->|---->|----|->|----->|->|
    assert_eq!(
	    Grupos::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
	    vec![
	        vec![1],
    	    vec![2, 2],
	        vec![1, 1],
	        vec![2, 2],
    	    vec![3],
	        vec![4, 4],
	        vec![3],
	    ]
    )
}

