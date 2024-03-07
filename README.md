<div>
<b>Расшифровка runanum:</b><br>
    ru  - russian<br>
    n   - nouns<br>
    a   - after<br>
    num - numbers<br>
</div><br> 

<div>
Небольшой растовый лефтпадик для корректных окончаний существительных после чисел:<br>
    3 яблока<br>
    5 яблок<br>
    21 яблоко<br>
    2557224 яблока<br>
</div><br> 

<div>
<i>Например:<i><br>

```rust
use runanum::Cases;

fn main() {
    let cases = Cases { 
        nom: "яблоко",
        gen: "яблока",
        plu: "яблок"
    };
    let result = runanum::crun(5, &cases);

    println!("5 {result}");
}
```
</div>
