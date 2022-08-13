use ringmaster::*;

#[test]
fn timeseries() {
    unsafe {
        let t = Timeseries::with_capacity(3);
        assert_eq!(t.len(), 0);
        assert_eq!(&t.snapshot_raw()[..], &vec![][..]);

        t.push(1);
        t.push(2);
        assert_eq!(&t.snapshot()[..], &vec![2, 1][..]);

        t.push(3);
        t.push(4);
        assert_eq!(&t.snapshot_raw()[..], &vec![4, 3, 2][..]);
    }
}
