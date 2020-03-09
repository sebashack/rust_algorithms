use std::cmp::Ordering;
use std::cmp::Ordering::Less;

pub fn selection_sort_by<T, F>(v: &mut Vec<T>, compare: F) -> &mut Vec<T>
    where F: Fn(&T, &T) -> Ordering
{
    let len = v.len();

    for i in 0..len {
        let mut min = i;

        for j in i+1..len {
            if let Less = compare(&v[j], &v[min]) {
                min = j;
            }
        }

        v.swap(min, i);
    }

    v
}

pub fn selection_sort<T>(v: &mut Vec<T>) -> &mut Vec<T>
    where T: Ord
{
    selection_sort_by(v, |v0, v1| v0.cmp(v1))
}

#[cfg(test)]
mod tests {
    use crate::algorithms::sorting::selection_sort_by;

    #[test]
    fn selection_sort_by_should_sort_the_vector() {
        let mut v = vec![12, 0, 3, 12, 23, 1, 9, 9, 10, 45, 6];

        selection_sort_by(&mut v, |n, m| n.cmp(m));

        let len = v.len();

        for (i, n) in v.iter().enumerate() {
            if i < len - 1 {
                assert!(n <= &v[i + 1]);
            }
        }
    }
}
