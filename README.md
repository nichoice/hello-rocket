# hello-rocket
rocket rocket_sync_db_pools simple demo
数据库使用 sqlite

## 环境
```bash
[dependencies]
rocket = {version = "0.5.0-rc.1", features = ["json"]}

[dependencies.rocket_sync_db_pools]
version = "0.1.0-rc.1"
features = ["diesel_sqlite_pool"]
```

## diesel_cli
安装 diesel cli tools
```bash
cargo install diesel_cli --no-default-features --features sqlite
```

## 编写sql
```bash
diesel setup
diesel migration generate article
```

会生成migrations 文件夹 当前时间_article\up.sql 跟 down.sql 文件

up.sql
```sql
CREATE TABLE user (
 id INTEGER NOT NULL PRIMARY KEY,
 username Text NOT NULL,
 password Text NOT NULL,
 issuper Text NOT NULL
)
```

down.sql
```sql
DROP TABLE user;
```

生成数据库
```bash
diesel migration run
```