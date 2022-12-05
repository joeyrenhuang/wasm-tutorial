<div align="center">

  <h1><code>wasm-培训-示例</code></h1>

  <strong align="left">
    WebAssembly(简称: wasm), 是针对机器模型，具有广泛应用的可执行格式. 它的设计初衷是为了实现便携,小,速度快(接近机器速度)的执行.  目前wasm在javascript与web社区中热度较高，不过wasm并没有规定在何种环境中使用，即wasm是一种“可移值的可执行文件"格式，可运行在各种上下文中。得益与wasm的运行效率, 当前的主要使用场景是javascript环境, 包括web和nodejs。
  </strong>


</div>

#### 目标

<sup>本文主要介绍如何将rust库编译成wasm并且发布到npm

#### 环境

 - #### [rust](https://www.rust-lang.org/tools/install)
    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh #类Unix环境 
    ```
 - #### [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
   <sup>wasm-pack是提代一站式将rust代码编译成wasm的工具，包括构建/测试/发布<sup>
    ```sh 
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh #类Unix环境
    ```
 - #### [npm](https://nodejs.org/en/)
   <sup>npm是使用最广泛的代码共享仓库, 将wasm-pack编译的.wasm发布到npm仓库</sup>

#### 模板

```sh
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name wasm-tutorial
cd wasm-tutorial
```

#### 构建

```sh
wasm-pack build
```
```
pkg/
├── package.json
├── README.md
├── wasm_tutorial_bg.was
├── wasm_tutorial.d.ts
└── wasm_tutorial.js

```

#### 测试

```
wasm-pack test --chrome
```

#### 发布

```
wasm-pack publish
```

#### 实验
<sup>将lib.rs替换成如下代码</sup>
```rs

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    "hello, wasm".to_string()
}
```

<sup>将tests/web.rs替换成如下代码</sup>
```rs

//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use wasm_tutorial::greet;

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(&greet()[..], "hello, wasm");
}
```

<sup>执行wasm-pack test --chrome, 并打开浏览器(http://127.0.0.1:8000/)查看测试结果</sup>
<br>
<sup>尝试修改test让测试失败，重新执行测试并查看浏览器相应测试失败信息<sup>

#### 实际使用
<sup>实际项目中假设使用webpack项目, 在根目录上新建www文件并跑一个webpack项目</sup>
```
npm init wasm-app www

```
<sup>将www/package.json文件里的hello-wasm-pack一行修改为本地依赖</sup>
```c
    - "hello-wasm-pack": "^0.1.0",
    + "wasm-tutorial": "file:../pkg",
```

<sup>将www/index.js作相应修改<sup>
```js
import * as wasm from "my-wasm";

alert(wasm.greet())
```

<sup>安装并启动</sup>
```sh
cd www
npm i
npm start 
```
<sup>打开浏览器(http://localhost:8080), 应该正确弹窗提示"hello, wasm"</sup>

#### 与js宿主环境交互
<sup> 一个简单实例是：wasm接收一个参数，将结果返回给js回调函数 </sup>
<br>
<sup>修改src/lib.rs为如下文件</sup>
```rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    "hello, wasm".to_string()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = wasmEnv, js_name = log)]
    fn log(o: JsValue);
}

#[wasm_bindgen(js_name = getAnswer)]
pub fn get_answer(s: &str) {
    log(match s {
        "hello, wasm" => JsValue::from("hello, browser"),
        _ => JsValue::from(JsError::new("Invalid Token!")),
    })
}
```

<sup>修改www/index.js<sup>
```js
import * as wasm from "my-wasm";

wasm.getAnswer(
  wasm.greet()
)

wasm.getAnswer("hello, world!")

```
<sup>修改www/bootstrap.js<sup>
```js
// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.
window.wasmEnv = {
  log (o) {
    console.log(o)
  }
}
import("./index.js")
  .catch(e => console.error("Error importing `index.js`:", e));
```
<sup>打开浏览器调试窗口, 期望结果如下</sup>
```js
hello, browser
bootstrap.js:6 Error: Invalid Token!
    at Module.__wbindgen_error_new (wasm_tutorial_bg.js:148:17)
    at __wbindgen_error_new (bootstrap.js:59:98)
    at e1ebdafeddf88c9f6301.module.wasm:0x328d
    at e1ebdafeddf88c9f6301.module.wasm:0x343c
    at Module.getAnswer (wasm_tutorial_bg.js:144:66)
    at eval (index.js:9:50)
    at ./index.js (0.bootstrap.js:46:1)
    at __webpack_require__ (bootstrap.js:87:30)
client:52 [WDS] Live Reloading enabled.
```
