// 定义一个 Student 结构体，包含以下字段：name、age、score
// 实现以下功能：
// - new(name: &str, age: u8, score: f32) -> Student：返回一个新的学生实例。
// - show(&self)：打印 Student 的信息，格式如 Name: Alice, Age: 18, Score: 95.5。
// - is_passed(&self) -> bool：如果 score >= 60.0 则返回 true，否则返回 false。
pub struct Student {
    pub name: String,
    pub age: u8,
    pub score: f32,
}

impl Student {
    pub fn new(name: &str, age: u8, score: f32) -> Self {
        Student {
            name: name.to_string(),
            age,
            score,
        }
    }

    pub fn show(&self) {
        println!(
            "Name: {}, Age: {}, Score: {:.1}",
            self.name, self.age, self.score
        );
    }

    pub fn is_passed(&self) -> bool {
        self.score >= 60.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_student_creation() {
        let student = Student::new("Alice", 18, 95.5);
        assert_eq!(student.name, "Alice");
        assert_eq!(student.age, 18);
        assert_eq!(student.score, 95.5);
    }

    #[test]
    fn test_student_is_passed_true() {
        let student = Student::new("Alice", 22, 75.0);
        assert!(student.is_passed());
    }

    #[test]
    fn test_student_is_passed_false() {
        let student = Student::new("Alice", 19, 50.0);
        assert!(!student.is_passed());
    }
}
