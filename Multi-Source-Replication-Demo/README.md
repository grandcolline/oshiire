# 手順

```bash
# とりあえず全て起動
$ docker-compose up -d

# master-aの事前作業
# databaseの作成・データの取り込み
$ mysql -uroot -h 127.0.0.1 -P 3306 -e "CREATE DATABASE master_database_a;"
$ mysql -uroot -h 127.0.0.1 -P 3306 -e "CREATE TABLE master_database_a.users (id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY, name VARCHAR(255) NOT NULL);"
$ mysql -uroot -h 127.0.0.1 -P 3306 -e "INSERT INTO master_database_a.users (id, name) VALUES (1, 'John'), (2, 'Paul'), (3, 'George');"
$ mysql -uroot -h 127.0.0.1 -P 3306 -e "SELECT * FROM master_database_a.users;"
+----+--------+
| id | name   |
+----+--------+
|  1 | John   |
|  2 | Paul   |
|  3 | George |
+----+--------+

# master-bの事前作業
# databaseの作成・データの取り込み
$ mysql -uroot -h 127.0.0.1 -P 3307 -e "CREATE DATABASE master_database_b;"
$ mysql -uroot -h 127.0.0.1 -P 3307 -e "CREATE TABLE master_database_b.users (id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY, name VARCHAR(255) NOT NULL);"
$ mysql -uroot -h 127.0.0.1 -P 3307 -e "INSERT INTO master_database_b.users (id, name) VALUES (1, 'Noel');"
$ mysql -uroot -h 127.0.0.1 -P 3307 -e "SELECT * FROM master_database_b.users;"
+----+------+
| id | name |
+----+------+
|  1 | Noel |
+----+------+


# master-aをロックする
$ mysql -uroot -h 127.0.0.1 -P 3306 -e "RESET MASTER;"
$ mysql -uroot -h 127.0.0.1 -P 3306 -e "FLUSH TABLES WITH READ LOCK;"
# master-aのDB情報をDumpする
$ mysqldump -uroot -h 127.0.0.1 -P 3306 master_database_a --master-data --single-transaction --flush-logs --events --column-statistics=0 > ./master_a_dump.sql

# master-bをロックする
$ mysql -uroot -h 127.0.0.1 -P 3307 -e "RESET MASTER;"
$ mysql -uroot -h 127.0.0.1 -P 3307 -e "FLUSH TABLES WITH READ LOCK;"
# master-bのDB情報をDumpする
$ mysqldump -uroot -h 127.0.0.1 -P 3307 master_database_b --master-data --single-transaction --flush-logs --events --column-statistics=0 > ./master_b_dump.sql


# slaveにデータを入れる
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "STOP SLAVE;"
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "CREATE DATABASE master_database_a;"
$ mysql -uroot -h 127.0.0.1 -P 3308 master_database_a < ./master_a_dump.sql
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "CREATE DATABASE master_database_b;"
$ mysql -uroot -h 127.0.0.1 -P 3308 master_database_b < ./master_b_dump.sql

# ログの確認
$ mysql -uroot -h 127.0.0.1 -P 3306 -e "SHOW MASTER STATUS;"
$ mysql -uroot -h 127.0.0.1 -P 3307 -e "SHOW MASTER STATUS;"

# slaveの設定変更
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "RESET SLAVE";
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "SET GLOBAL master_info_repository = 'TABLE';"
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "SET GLOBAL relay_log_info_repository = 'TABLE';"

# 上で確認したログを入れる
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "CHANGE MASTER TO MASTER_HOST='master-a', MASTER_USER='root', MASTER_PASSWORD='', MASTER_PORT=3306, MASTER_LOG_FILE='bin-log.000002', MASTER_LOG_POS=154, MASTER_CONNECT_RETRY=10 for channel 'master-a';"
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "CHANGE MASTER TO MASTER_HOST='master-b', MASTER_USER='root', MASTER_PASSWORD='', MASTER_PORT=3306, MASTER_LOG_FILE='bin-log.000002', MASTER_LOG_POS=154, MASTER_CONNECT_RETRY=10 for channel 'master-b';"

# レプリケーション起動
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "START SLAVE FOR CHANNEL 'master-a';"
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "START SLAVE FOR CHANNEL 'master-b';"

# masterをunlockする
$ mysql -u root -h 127.0.0.1 -P 3306 -e "UNLOCK TABLES;"
$ mysql -u root -h 127.0.0.1 -P 3307 -e "UNLOCK TABLES;"

# slaveの状態確認
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "SHOW SLAVE STATUS FOR CHANNEL 'master-a'\G"
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "SELECT * FROM master_database_a.users;"
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "SHOW SLAVE STATUS FOR CHANNEL 'master-b'\G"
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "SELECT * FROM master_database_b.users;"

# INSERT
$ mysql -uroot -h 127.0.0.1 -P 3306 -e "INSERT INTO master_database_a.users (id, name) VALUES (4, 'Ringo');"
$ mysql -uroot -h 127.0.0.1 -P 3307 -e "INSERT INTO master_database_b.users (id, name) VALUES (2, 'Liam');"

# slaveの状態確認あげいん
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "SHOW SLAVE STATUS FOR CHANNEL 'master-a'\G"
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "SELECT * FROM master_database_a.users;"
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "SHOW SLAVE STATUS FOR CHANNEL 'master-b'\G"
$ mysql -uroot -h 127.0.0.1 -P 3308 -e "SELECT * FROM master_database_b.users;"
```
