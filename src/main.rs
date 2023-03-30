mod snippet_1;
mod snippet_2;

fn main() {
    snippet_1::set_point(1, 2, 3);
    snippet_1::set_point(2, 5, 5);
    snippet_1::print_points();

    snippet_2::count_in_threads();
}
