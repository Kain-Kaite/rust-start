fn main() {
    let mut page = 100;
    println!("If there was a frind of us who opens the page, everytime larger than the given page by 2, of any books,");
    println!("What will be the next page, we would get?\n");
    println!("First, I opened the page {page}");

    let mut page = { page + 2 };
    println!("Now My friend opened the book, the page is {page}");
    println!("Wow. Perfect thanks buddy.\n");

    let friend_page = {
                            let my_page = page;
                            println!("Let's try once again. I opened the book as I always do. Page {my_page}");
                            my_page + 2
    };
    println!("My friend opened the book, page {friend_page}");
    println!("Friend, You are the best.\n");
}
