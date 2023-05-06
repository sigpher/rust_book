pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[cfg(test)]

#[derive(Debug)]
pub struct Rectangle {
  pub   witdh: u32,
    pub height: u32,
}

mod tests {
    use super::*;
    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        let result = add(2, 2);
        assert_eq!(result, 5);
    }

    
}
