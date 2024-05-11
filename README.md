# Godot + Rust言語でローグライクゲームを作る

- Author: Hiroaki Yaguchi, 947D-Tech

## はじめに

本プロジェクトは「クシナダ機巧株式会社」が運営するVTuber「輪廻ヒロ」が配信中に制作している
Godot + Rust言語でのローグライクゲームのプロジェクトとなります。
その性質上、issueやPReqへの対応が難しい場合がありますがご了承いただけますと幸いです。

## 実行方法

### 動作確認環境について

Windows11上のGodot 4.2.1で動作を確認しています。

Rust言語の開発環境をインストールしてください。

### Rustエクステンションのコンパイル

extension/roguelike_extensionの下で`cargo build`してください。

### プロジェクトの実行

Godotを開き、インポート->本リポジトリのprojectを選択し、
プロジェクトを開いてください。


## ライセンス

MITライセンスで公開します。
ただし、使用している各素材については別途ライセンスに従ってください。

MPLUSフォントはSIL Open Font Licenseです。
ライセンスはproject/fontsの下にも格納してあります。

models以下の.blendファイル(自作)はCC0とします。
