trait Permute <T>{
    fn permutate(&self) -> Vec<Vec<T>>;
    fn _permute(&self, acc: &mut Vec<Vec<T>>, k: usize, values: &mut Vec<T>) -> Vec<Vec<T>>;
}

// Needs work! 

impl<T> Permute<T> for Vec<T> where T: Copy {
    // sneaky heaps implementation
    fn permutate(&self) -> Vec<Vec<T>> {
        let mut list = self.clone();
        self._permute(&mut vec![list.clone()], self.len(), &mut list)
    }
    
    fn _permute(&self, acc: &mut Vec<Vec<T>>, k: usize, values: &mut Vec<T>) -> Vec<Vec<T>> {
        if k == 1 {
            return vec![values.clone()]
        } else {
            self._permute(acc, k - 1, values);

            for i in 0..(k - 1) {
                if i % 2 == 0 {
                    let tmp : T = values[i]; 
                    values[i] = values[k-1];
                    values[k-1] = tmp;
                } else {
                    let tmp : T = values[0]; 
                    values[0] = values[i];
                    values[i] = tmp;
                }
                self._permute(acc, k - 1, values);
            }
            acc.push(values.clone());
            return acc.clone();
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        let vec = vec![1,2,3];

        assert_eq!(vec.permutate(), vec![
            vec![1,2,3],
            vec![2,1,3],
            vec![1,3,2],
            vec![3,2,1],
            vec![3,1,2],
            vec![2,3,1],
        ])
        
    }
} 
