use super::SingleLinkedList;
use crate::core::List;

pub struct IntoIter<T> {
    list: SingleLinkedList<T>,
}

impl<T> IntoIter<T> {
    pub fn new(list: SingleLinkedList<T>) -> Self {
        Self { list }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.list.is_empty() {
            None
        } else {
            self.list.pop_front()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list_iterators() {
        let mut list: SingleLinkedList<i32> = SingleLinkedList::new();

        // Итератор по ссылкам
        {
            let mut iter = list.iter();
            assert_eq!(iter.next(), None);
        }

        // Изменяемый итератор
        {
            let mut iter_mut = list.iter_mut();
            assert_eq!(iter_mut.next(), None);
        }

        // IntoIterator (забирает владение)
        let into_iter = list.into_iter();
        assert_eq!(into_iter.collect::<Vec<_>>(), Vec::<i32>::new());
    }

    #[test]
    fn test_sequential_iteration() {
        let mut list = SingleLinkedList::new();
        for i in 0..5 {
            list.push(i);
        }

        // Проверка итератора по ссылкам
        let collected: Vec<_> = list.iter().collect();
        assert_eq!(collected, vec![&0, &1, &2, &3, &4]);

        // Проверка изменяемого итератора (изменяем значения)
        for item in list.iter_mut() {
            *item *= 2;
        }
        let doubled: Vec<_> = list.iter().collect();
        assert_eq!(doubled, vec![&0, &2, &4, &6, &8]);

        // Проверка IntoIterator
        let into_collected: Vec<_> = list.into_iter().collect();
        assert_eq!(into_collected, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_partial_iteration() {
        let mut list = SingleLinkedList::new();
        for i in 0..10 {
            list.push(i);
        }

        {
            let mut iter = list.iter();
            // Проходим только первые 3 элемента
            assert_eq!(iter.next(), Some(&0));
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
            // Останавливаемся, не проходя до конца
        }

        // Список должен остаться целым
        assert_eq!(list.len(), 10);

        // Можно начать итерацию заново
        let first_element = list.iter().next();
        assert_eq!(first_element, Some(&0));
    }

    #[test]
    fn test_concurrent_iterators() {
        let mut list = SingleLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // Создаём несколько итераторов одновременно
        let collect1: Vec<_> = list.iter().cloned().collect();
        let collect2: Vec<_> = list.iter().cloned().collect();
        {
            let mut iter3 = list.iter_mut();
            let first_mut = iter3.next().unwrap();
            *first_mut = 100;
        }

        assert_eq!(collect1, vec![1, 2, 3]);
        assert_eq!(collect2, vec![1, 2, 3]);
        assert_eq!(list.iter().collect::<Vec<_>>(), vec![&100, &2, &3]);
    }

    #[test]
    fn test_mutable_iteration_modification() {
        let mut list = SingleLinkedList::new();
        for i in 1..=3 {
            list.push(i);
        }

        let mut counter = 0;
        for item in list.iter_mut() {
            counter += 1;
            *item = *item * counter; // Умножаем на номер итерации
        }

        let result: Vec<_> = list.iter().copied().collect();
        assert_eq!(result, vec![1, 4, 9]); // 1×1, 2×2, 3×3
    }

    #[test]
    fn test_large_list_iteration() {
        const LARGE_SIZE: usize = 10_000;

        let mut list = SingleLinkedList::new();
        for i in 0..LARGE_SIZE {
            list.push(i);
        }

        // Полный проход через итератор
        let sum: usize = list.iter().sum();
        let expected_sum: usize = (0..LARGE_SIZE).sum();
        assert_eq!(sum, expected_sum);

        // Частичная итерация с take()
        let first_10: Vec<_> = list.iter().take(10).copied().collect();
        assert_eq!(first_10, (0..10).collect::<Vec<_>>());
    }
}
