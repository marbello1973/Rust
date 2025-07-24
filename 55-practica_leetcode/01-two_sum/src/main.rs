
// sumar los numeros del target

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{

    let mut indices_map: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
    

    for (i, &n) in nums.iter().enumerate() {
        let c = target - n;

        //let c = nums[n] + nums[n + 1]


        println!("i: {}, c: {}", i, c);
        
       if let Some(&indice) = indices_map.get(&c) {
            return vec![indice as i32, i as i32];
        }

        //indices_map.insert(vec!(indice), vec!(i));

       indices_map.insert(n, i); 

    } 
       
   vec![]
}
fn main() {
    
    let two = two_sum([3,2,4].to_vec(), 6 );
    println!("numeros: {:?}", two);    
}




