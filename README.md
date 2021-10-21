# 예비 개발자는 RUST의 꿈을 꾸지 않는다

모든 환경은 windows10에서 진행되었습니다. Linux혹은 MacOS 유저는 개발환경 구축 전까지는 다른 글을 참조해주시기 바랍니다

#### 00. 개발환경 구축 + 테스트

 https://www.rust-lang.org/en-US/install.html

위 사이트에서 인스톨러 받고 영어 잘 읽고 기다리면 된다.

vs code를 활용한 작업을 위해 아래 확장 프로그램을 설치하자.

![image-20211018233405403](image-20211018233405403.png)

바로 cargo를 활용한 프로젝트 생성으로 넘어가자

```
cargo new "파일명" --bin
```

으로 프로젝트를 생성하면 프로젝트 생성 끝이다. src/main.rs은 

```
fn main() {
    println!("Hello, world!");
}
```

기본적인 코드가 짜여져있다.

이제 빌드와 실행을 해보자

```
cargo build
cargo run
---------
cargo check
```

을 통해 빌드와 실행 둘다 된다. 

정상적인 환경이라면 Hello, world! 가 바로 뜬다. 

올바르게 컴파일이 되었는지만 확인하려면 빌드 이후 cargo check를 해주면 된다.

가끔 

```
warning: crate `test_Cargo` should have a snake case name
```

와 같은 경고가 뜨는 경우를 봤는데

main.rs 에 \#![allow(non_snake_case)] 를 추가해주면 해결된다.

------

#### 01. 기본 문법-1 (변수 , 데이터 타입)

##### 변수:

let 

특징: 불변성 변수이기에 재할당이 불가능하다

let mut

let에서 가변성이 되는 변수 

const

상수 더 할말이 없다

shadowing이라는 방법이 있는데 

```
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
```

이런 코드다. 결과는 12가 나온다. let mut와의 차이점이라면 새롭게 정의하는 것이라는 점이다.

또한 let mut는 변수의 형식을 바꿀 수 없다. 무슨 말이냐면 let mut int로 선언한 a라는 함수에 string값을 넣을수 없다는 사실이다. ~~쓰고 보니 너무 당연한거 같기도하네~~

##### 데이터 타입: 

```
#![allow(non_snake_case)]
fn main() {
    // addition
    let sum = 5 + 10;
    println!("{}",sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("{}",difference);
    // multiplication
    let product = 4 * 30;
    println!("{}",product);
    // division
    let quotient = 56.7 / 32.2;
    println!("{}",quotient);
    // remainder
    let remainder = 43 / 5;
    println!("{}",remainder);
}
```

이런 형태의 코드에서 결과는 다음과 같다

```
15
91.2
120
1.7608695652173911
8
```



튜플, 배열과 같은 복합 타일 역시 자세히 구현되어있다.

```
#![allow(non_snake_case)]
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

```
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
```

------

#### 02. 기본 문법-  2 (함수 , 제어문)

함수 선언은 fn으로 시작하며 함수 이름 뒤에 괄호 형태로 인자를 받는다. 

```
#![allow(non_snake_case)]

fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

아래와 같이 사용 가능하다

```
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

여러개의 인자 역시 가능하다 

c언어를 처음 배울때 많이 헷갈렸던 구문과 표현식은 다음과 같다

```
#![allow(non_snake_case)]

fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

실행결과는 4가 나온다.  유의해야할점은 표현식에서의 종결은 ;이 존재하지 않다는 점이다. 

값을 return하는 함수는 다음과 같이 작성한다 

```
#![allow(non_snake_case)]

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

주석은 `//주석입니다` 로 작성가능하다

다음은 기본적인 조건문이다.

 c++ , 파이썬 문법과 적절히 섞은 느낌이 든다 

```
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

러스트에서 가장 흥미로운 부분중 하나였던 let에서의 if문이다

```
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6 //"six" 로 작성한다면 if와 else의 결과 값이 다르기에 오류가 난다 
    };

    println!("The value of number is: {}", number);
}
```

조건문 다음으로 항상 나오는 반복문이다

loop문, while문과 for문이 존재한다

loop문은 간단하게 무한 반복하는 반복문이다. 딱히 쓸모가 크게 있어보이지 않는다..

```
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

```

if문 조건 작성하듯이 while문의 탈출조건을 작성하면 된다. 

```
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

```

다음은 for 문이다 파이썬의 코드와 같이 작성하면 된다 

```
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

```

리스트를 역정렬(reverse) 해주는 rev 함수를 사용하여 간단한 카운트다운 프로그램도 작성할 수 있다. 

------

#### 03. 러스트의 특성 - 소유권

소유권(Ownership)은 러스트가 가비지 콜렉터 없이 메모리 안정성을 보장하게 해주는 방법입니다. 이외 빌림, 슬라이스, 메모리에 데이터를 저장하는 방법에 대해 다루겠다.

러스트의 소유권 규칙은 다음과 같다

1. 러스트의 각각의 값은 해당값의 *오너*(*owner*)라고 불리우는 변수를 갖고 있다.
2. 한번에 딱 하나의 오너만 존재할 수 있다.
3. 오너가 스코프 밖으로 벗어나는 때, 값은 버려진다(dropped).

다음 코드는 문제가 발생한다. 

```
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

그 이유는 s2에서 s1의 값이 "복사"가 아닌 이동이 되었기 때문이다.

복사는 다음과 같이 clone 이라는 방법을 사용하면 된다 

```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

```

#![allow(non_snake_case)]
fn main() {
    let s = String::from("hello");  

    takes_ownership(s);     
        
    let x = 5;                     

    makes_copy(x);                  
    println!("{}",x);
} 

fn takes_ownership(some_string: String) { 
    println!("{}", some_string);
} 

fn makes_copy(some_integer: i32) { 
    println!("{}", some_integer);
} 
```

위와 같은 코드는 정상적으로 컴파일&실행이 된다.

다음은 다뤄볼 내용은 참조자다. 참조자는 "&변수명"으로 참조가 가능하다. 다음 코드를 보자

```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

이렇게 값을 참조가 가능하다. 

```
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

하지만 위 코드는 문제가 생긴다. 그 이유는 참조한 값을 변경/ 추가 할려 하기 때문이다. 

다음코드를 수정하는 방법은 mut 태그를 추가하는 방법이다. 이렇게 가변 참조자를 사용하면 한가지 문제가 생기는데 특정 스코프에는 오직 한가지에만 가변 참고자만 생성 가능하다는 사실이다. 

```
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

바로 이런 형태의 코드가 안된다는 말이다. 

```

#![allow(unused)]
fn main() {
let mut s = String::from("hello");

{
    let r1 = &mut s;

}

let r2 = &mut s;
}
```

이렇게 고쳐주면 된다. 대신 r1은 스코프 종료 이후 사용 불가능하다는 점을 알아야 한다. 

다음은 댕글링 참조자에 대해 배워보자. 댕글링 포인터는 어떤 메모리를 가리키는 포인터를 보존하는 동안, 그 메모리를 해제함으로써 다른 개체에게 사용하도록 줘버렸을 지도 모를 메모리를 참조하고 있는 포인터를 말한다. 

```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

"hello"라는 문자열을 반환하고 싶은데 위코드에서는 댕글링 오류로 인해 안된다. 해결하기 위해서는 

```
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

위 형태의 함수를 만들어서 관리 해주면 된다.

다음은 슬라이싱이다. 문자열에서 특정 부분만 짜르기 위해 슬라이싱을 진행해주면 되는데

```
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

다음과 같은 형태로 작성하면 된다. 

```
#![allow(unused)]
fn main() {
    let arr = [1,2,3,4];
    let brr = &arr[0..2];
    println!("{}",brr[0]);
}
```

배열에서는 위와 같이 사용하면 된다.
