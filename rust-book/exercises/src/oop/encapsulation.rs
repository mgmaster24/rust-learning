use crate::model::AveragedCollection;

pub fn exercise() {
    let mut averaged_collection = AveragedCollection::new();
    averaged_collection.add(24);
    averaged_collection.add(69);
    averaged_collection.add(128);
    averaged_collection.add(32);
    averaged_collection.add(16);

    println!("Average: {}", averaged_collection.average());

    let result = averaged_collection.remove();
    if let Some(v) = result {
        println!("Rewoved: {}", v)
    }

    println!("Average: {}", averaged_collection.average());
}
