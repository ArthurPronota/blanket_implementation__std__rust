/*

1) Реализация Clone для всех типов, реализующих Copy:

impl<T: Copy> Clone for T {
    fn clone(&self) ->Self {
        *self   // простое побитовое копирование
    }
}

2) Для всех ссылок &T (независимо от T):
Реализация Clone для ссылок на тип который может быть как Sized так и ?Sized

impl<T: ?Sized> Clone for &T {
    fn clone(
        &self   // Это ссылка на Self => &Self => &&T
       ) -> Self {
        *self   // *self => *&Self => Self => &T
    }
}

*/
struct Celsius(f64) ;

#[derive(Debug)]
struct Fahrenheit(f64) ;

impl From<Celsius> for Fahrenheit {
    fn from(value: Celsius) -> Self {
        Self(
            value.0 * 1.8 + 32.
        )
    }
}

fn main() {
    let fah: Fahrenheit = Celsius(10.).into() ;
    println!("fah: {:?}", fah) ;    // Out: fah: Fahrenheit(50.0)

    let fah2 = Fahrenheit::from(Celsius(10.)) ;
    println!("fah2: {:?}", fah2) ;  // Iut: fah2: Fahrenheit(50.0)
}
