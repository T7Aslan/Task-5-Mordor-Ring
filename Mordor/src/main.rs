// 1. Определяем структуру данных

#[derive(Debug)]
pub struct RingBuffer {
    buffer: Vec<Option<u8>>, // Хранилище данных (None - пустая ячейка)
    capacity: usize,         // Максимальная вместимость буфера
    head: usize,             // Индекс для чтения (голова)
    tail: usize,             // Индекс для записи (хвост)
    size: usize,             // Текущее количество элементов
}

// 2. Реализация методов
impl RingBuffer {
    // 2.1. Создаём новый буфер заданного размера
    pub fn new(capacity: usize) -> Self {
        // Проверяем, что размер буфера положительный
        assert!(capacity > 0, "В буфере есть место!");

        RingBuffer {
            buffer: vec![None; capacity], // Инициализируем пустыми значениями
            capacity,                     // Сохраняем ёмкость
            head: 0,                      // Начинаем с индекса 0
            tail: 0,                      // Начинаем с индекса 0
            size: 0,                      // Начальный размер - 0
        }
    }

    // 2. Проверка на пустоту
    pub fn is_empty(&self) -> bool {
        self.size == 0 // Если размер 0 - буфер пуст
    }

    // 3. Проверка на заполненность
    pub fn is_full(&self) -> bool {
        self.size == self.capacity // Если размер равен ёмкости - буфер полон
    }

    // 4. Текущее количество элементов
    pub fn len(&self) -> usize {
        self.size // Просто возвращаем размер
    }

    // 5. Запись элемента
    pub fn push(&mut self, value: u8) -> Result<(), String> {
        if self.is_full() {
            return Err("Буфер Заполнен!".to_string()); // Ошибка если полон
        }

        self.buffer[self.tail] = Some(value); // Записываем значение
        self.tail = (self.tail + 1) % self.capacity; // Перемещаем хвост с закольцовыванием
        self.size += 1; // Увеличиваем размер
        Ok(()) // Возвращаем успешный результат
    }

    // 2.6. Чтение элемента
    pub fn pop(&mut self) -> Option<u8> {
        if self.is_empty() {
            return None; // Возвращаем None если пуст
        }

        let value = self.buffer[self.head].take(); // Забираем значение из головы
        self.head = (self.head + 1) % self.capacity; // Перемещаем голову
        self.size -= 1; // Уменьшаем размер
        value // Возвращаем значение
    }

    // 2.7. Запись нескольких элементов
    pub fn extend(&mut self, data: &[u8]) -> usize {
        let mut count = 0;
        for &byte in data {
            if self.push(byte).is_err() {
                // Пытаемся добавить каждый байт
                break; // Прерываем если буфер полон
            }
            count += 1; // Считаем успешно добавленные
        }
        count // Возвращаем количество добавленных
    }

    // 2.8. Чтение нескольких элементов
    pub fn drain(&mut self, count: usize) -> Vec<u8> {
        let mut result = Vec::new();
        for _ in 0..count {
            match self.pop() {
                // Пытаемся извлечь элемент
                Some(byte) => result.push(byte), // Добавляем в результат
                None => break,                   // Прерываем если буфер пуст
            }
        }
        result // Возвращаем прочитанные байты
    }
}

// 3. Модуль тестирования
#[cfg(test)]
mod tests {
    use super::RingBuffer;

    #[test]
    fn test_creation() {
        let rb = RingBuffer::new(5);
        assert_eq!(rb.capacity, 5);
        assert!(rb.is_empty());
        assert_eq!(rb.len(), 0);
    }

    #[test]
    fn test_basic_operations() {
        let mut rb = RingBuffer::new(3);

        // Тест записи
        assert!(rb.push(1).is_ok());
        assert!(rb.push(2).is_ok());
        assert_eq!(rb.len(), 2);
        assert!(!rb.is_empty());
        assert!(!rb.is_full());

        // Тест чтения
        assert_eq!(rb.pop(), Some(1));
        assert_eq!(rb.pop(), Some(2));
        assert_eq!(rb.pop(), None);
        assert!(rb.is_empty());
    }

    #[test]
    fn test_full_behavior() {
        let mut rb = RingBuffer::new(2);

        assert!(rb.push(1).is_ok());
        assert!(rb.push(2).is_ok());
        assert!(rb.push(3).is_err()); // Должно вернуть ошибку

        assert_eq!(rb.drain(3), vec![1, 2]); // Читаем больше чем есть
    }

    #[test]
    fn test_wrap_around() {
        let mut rb = RingBuffer::new(3);

        // Заполняем и частично освобождаем
        assert_eq!(rb.extend(&[1, 2, 3]), 3);
        assert_eq!(rb.drain(2), vec![1, 2]);

        // Должны записаться в начало
        assert_eq!(rb.extend(&[4, 5]), 2);
        assert_eq!(rb.drain(3), vec![3, 4, 5]);
    }

    #[test]
    fn test_edge_cases() {
        let mut rb = RingBuffer::new(1);

        // Граничный случай с буфером размером 1
        assert!(rb.push(1).is_ok());
        assert!(rb.is_full());
        assert!(rb.push(2).is_err());

        assert_eq!(rb.pop(), Some(1));
        assert!(rb.is_empty());
    }
}

// 4. Пример использования
fn main() {
    let mut buffer = RingBuffer::new(3);

    // Заполняем буфер
    buffer.push(10).unwrap();
    buffer.push(20).unwrap();
    buffer.push(30).unwrap();

    // Попытка переполнения
    if buffer.push(40).is_err() {
        println!("Буффер Заполнен!");
    }

    // Читаем данные
    while let Some(value) = buffer.pop() {
        println!("Прочитанны!: {}", value);
    }
}
