<div align="center">

# Rust Learning

**Rust 語言學習筆記與實作紀錄**

從基礎語法開始，透過實際編寫程式、拆解問題與反覆練習，逐步理解 Rust 的語言設計與核心概念。

<p>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-Language-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust"></a>
  <a href="https://doc.rust-lang.org/cargo/"><img src="https://img.shields.io/badge/Cargo-Build%20Tool-555555?style=for-the-badge&logo=rust&logoColor=white" alt="Cargo"></a>
  <img src="https://img.shields.io/badge/Status-Learning-informational?style=for-the-badge" alt="Learning status">
</p>

</div>

## 簡介

這個 repository 用來保存我的 Rust 學習過程，包含教材實作、基礎題目、程式碼註解與解題思路。

學習重點不只是讓程式成功執行，也包括理解語法背後的概念、先自行思考解法，再透過實作與驗證修正理解。部分程式碼保留學習當下的註解與嘗試，不一定代表最佳實踐。

## 專案結構

| 目錄 | 學習內容 |
| --- | --- |
| [`hello_rust/`](hello_rust/) | 變數與可變性、條件判斷、迴圈、陣列、函式及 Slice 的基礎練習 |
| [`v02_cargo/`](v02_cargo/) | Cargo 專案結構、`Cargo.toml` 與 Hello World |
| [`v03_game/`](v03_game/) | 猜數字遊戲：隨機數、標準輸入、型別轉換、`match` 與 `Ordering` |

每個目錄是獨立的 Cargo 專案，可分別進入目錄執行。

## 開始使用

**前置需求：** 安裝 [Rust 與 Cargo](https://www.rust-lang.org/tools/install)。

```bash
git clone https://github.com/iamrivershenshuhe/rust-learn.git
cd rust-learn

# 進入其中一個練習專案
cd hello_rust

# 編譯並執行
cargo run
```

其他練習可切換到對應目錄，使用 `cargo run` 執行。

## 學習主題

目前 repository 涵蓋以下基礎概念：

- **基本語法：** 變數、`mut`、條件判斷與迴圈
- **資料與函式：** 陣列、Slice、函式參數及回傳值
- **所有權相關入門：** 借用與參照（`&`、`&mut`）
- **輸入與控制流程：** `std::io`、`match`、`loop` 與 `break`
- **工具與相依套件：** Cargo、`Cargo.toml` 與 `rand` crate

這是持續更新的學習紀錄；上述項目代表已接觸或練習的內容，不表示完全掌握。

## 學習資源

以下是主要參考的教材：

| 資源 | 說明 |
| --- | --- |
| [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) | Rust 官方教材（The Book），作為語言基礎與核心觀念的參考 |
| [BEATAI Rust Course](https://beatai.org/rust-course/about-book) | 中文 Rust 學習教材，輔助理解與練習 |

程式碼與註解主要用於記錄個人理解及實作歷程，並非教材內容的完整重製。

## 延伸學習資源

以下整理其他值得探索的 Rust 教材，作為日後複習與進階學習的參考；**列在這裡不代表已完成學習**。

### 官方教材與互動練習

- [**The Rust Programming Language**](https://doc.rust-lang.org/book/) — Rust 官方入門教材（也列於上方主要學習資源）。
- [**Rust By Example**](https://doc.rust-lang.org/rust-by-example/) — 以可執行範例理解 Rust 語法及概念。
- [**The Rust Programming Language: Experimental Edition**](https://rust-book.cs.brown.edu/) — 提供互動測驗，以及 Ownership 等概念的補充說明。
- [**Rust Development Classes**](https://rust-classes.com/) — 偏重動手實作的範例與練習。

### 不同背景與深入學習

- [**Comprehensive Rust — Google**](https://google.github.io/comprehensive-rust/) — Google Android 團隊開發的免費課程，從基礎語法延伸至泛型、錯誤處理、並行及系統領域。
- [**Yet Another Rust Resource (YARR!)**](https://yet-another-rust-resource.pages.dev/introduction) — 適合已有高階語言經驗、希望接觸較低階程式設計概念的學習者。
- [**The Rust Book (Abridged)**](https://jasonwalton.ca/rust-book-abridged/) — 精簡版 The Rust Book，方便已有其他程式語言基礎的人快速閱讀。
- [**Idiomatic Rust Snippets**](https://idiomatic-rust-snippets.org/) — Rust 常見語法、設計模式與演算法的簡短範例。
- [**Rust Training Books — Microsoft**](https://microsoft.github.io/RustTraining/) — 從 C/C++、C#、Python 等語言背景切入 Rust 的教材。
- [**Learning Rust**](https://learning-rust.github.io/) — 面向不同程度讀者的 Rust 教學資源。

## 學習方式

**理解概念 → 自行思考 → 動手實作 → 驗證與修正**

希望透過持續累積練習，建立獨立分析問題與編寫程式的能力，而不只是記住語法。

---

<div align="center">
  <sub>Personal Rust learning journey · Work in progress</sub>

  <p><img src="https://raw.githubusercontent.com/iamrivershenshuhe/iamrivershenshuhe/main/patrick-jane.gif" alt="Patrick Jane" width="420"></p>
</div>
