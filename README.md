# 예비 개발자는 RUST의 꿈을 꾸지 않는다

모든 환경은 windows10에서 진행되었습니다. Linux혹은 MacOS 유저는 개발환경 구축 전까지는 다른 글을 참조해주시기 바랍니다

#### 00. 개발환경 구축 + 테스트

 https://www.rust-lang.org/en-US/install.html

위 사이트에서 인스톨러 받고 영어 잘 읽고 기다리면 된다.

vs code를 활용한 작업을 위해 아래 확장 프로그램을 설치하자.

![image-20211018233405403](C:\Users\wesle\5lues\develop\Rust\Rust_Study\image-20211018233405403.png)

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

