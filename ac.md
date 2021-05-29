## 受け入れ条件
### 新しくファイルが置かれたときにCreateの通知が表示されること
- Given: アプリケーションが実行されている
- When: 確認対象のディレクトリに新しくファイルを作成する
- Then: 新しく作成されたファイル名とCreateイベントであることを示すメッセージが表示される

### ファイル名を変更したときにRenameの通知が表示されること
- Given: アプリケーションが実行されている
- When: 確認対象のディレクトリに存在するファイルの名前を変更する
- Then: 変更された後のファイル名とRenameイベントであることを示すメッセージが表示される