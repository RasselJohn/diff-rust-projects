// Нач. позиция (1000,1000), лимит=25
// ниже и/или левее уйти нельзя - 9+9+9=27>=25

// при лимит=1 - стоит на месте - колво посещ клеток  1(текущ клетка)
// при лимит=2 - стоит на месте - колво посещ клеток 1(текущ клетка)
// при лимит=3 - либо влево либо вверх - колво посещ клеток 3
// при лимит=4  - колво посещ клеток 6 - где 6=3+2+1
// при лимит=5  - колво посещ клеток 10 - где 10 = 4+3+2+1
// при лимит=6  - колво посещ клеток 15 - где 15 = 5+4+3+2+1
// ...
// при лимит=25  - колво посещ клеток = 24+23+22+...+1

// то бишь муравей при заданном ограничении может посетить колво клеток
// равное сумме арифм прогрессии предыдущ знач суммы цифр до единицы включит.
// то есть мурав может посетить при огранич 25 => s=n*(a1+an)/2 => s = 24 * (1 + 24)/2 = 300 шагов суммарно

// Пример как выглядит для лимит = 6 есть в example.png файле - точками выделены перемещения муравья

fn main() {
    let max_digits_sum = 25;
    let max_cells = ((max_digits_sum - 1) * (1 + (max_digits_sum - 1))) / 2;
    println!("Max cells={}", max_cells);
}