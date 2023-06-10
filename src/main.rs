// yewフレームワークをインポート
use yew::prelude::*;
use yew_router::prelude::*;

// モジュールを定義
mod components;
mod content;
mod generator;
mod pages;

// 他のモジュールから特定のコンポーネントや関数をインポート
use pages::author::Author;
use pages::author_list::AuthorList;
use pages::home::Home;
use pages::page_not_found::PageNotFound;
use pages::post::Post;
use pages::post_list::PostList;
use yew::html::Scope;

// 特定のトレイトの標準的な実装を提供
// Routableのみyew-routerライブラリのトレイト
#[derive(Routable, PartialEq, Eq, Clone, Debug)]


