# clickpipe-rs 🔧➡️📊

**Fault-tolerant event ingester для ClickHouse на Rust**  
Локальная буферизация на диск, батчинг, retry с exponential backoff и дедупликация событий.

---

## 🏗️ Архитектура

```
[Источники] ──► [Валидация] ──► [Буфер (диск)] ──► [Batching] ──► [ClickHouse]
                      │                                            │
                      ▼                                            ▼
                 Метрики / Логи                              Retry + Dedup
```

**Компоненты:**
| Модуль | Назначение |
|---|---|
| `source` | Чтение сырых событий (JSONL, в будущем — HTTP, Kafka) |
| `validate` | Нормализация и проверка полей |
| `sink` | Отправка в ClickHouse через HTTP (JSONEachRow) |
| `model` | Типы `RawEvent` / `NormalizedEvent` |
| `error` | Централизованные типы ошибок |

---

## 🚀 Быстрый старт

### Требования
- Rust (edition 2024)
- Docker (для ClickHouse)

### 1. Запустите ClickHouse
```sh
docker compose up -d
```

### 2. Соберите и запустите ingester
```sh
cd clickpipe-rs
cargo run -- --name World
```

### 3. Или соберите release-версию
```sh
cargo build --release
./target/release/clickpipe-rs.exe --name World
```

---

## 🗺️ Roadmap

### ✅ v0.1.0 — MVP (текущее)
- [x] Чтение JSONL-файлов
- [x] Валидация и нормализация событий
- [x] Хеширование для дедупликации
- [x] Отправка в ClickHouse (JSONEachRow)
- [x] Docker Compose для ClickHouse
- [x] Clap CLI (базовый)

### 🟡 v0.2.0 — Настоящий CLI и конфигурация
- [x] Аргументы: `--input-file`, `--clickhouse-url`, `--batch-size`, `--flush-interval`
- [x] TOML/YAML конфиг-файл
- [x] Переменные окружения как fallback
- [ ] Graceful shutdown (Ctrl+C)

### 🟠 v0.3.0 — Fault Tolerance
- [ ] Дисковая буферизация (SQLite / файлы) при недоступности ClickHouse
- [ ] Retry с exponential backoff
- [ ] Batching по размеру и по времени
- [ ] In-memory дедупликация (bloom filter)

### 🔵 v0.4.0 — Мониторинг и логи
- [ ] Структурированное логирование (tracing crate)
- [ ] Prometheus-метрики: `events_received`, `events_sent`, `events_dropped`, `buffer_size`
- [ ] Healthcheck endpoint

### 🟢 v0.5.0 — Production-ready
- [ ] Множественные источники: HTTP-endpoint, Kafka consumer, NATS
- [ ] Поддержка форматов: Avro, Protobuf (опционально)
- [ ] HTTPS/TLS
- [ ] Dockerfile для самого ingester
- [ ] Kubernetes manifests / Helm chart

### 🔮 v1.0.0 — Стабильный релиз
- [ ] Persistent дедупликация
- [ ] Rate limiting / backpressure
- [ ] Rate limiting на стороне ClickHouse
- [ ] CLI автодополнение (shell completions)
- [ ] Документация и benchmarks
- [ ] CI/CD (GitHub Actions: build, test, clippy, fmt)

---

## 🧪 Тестирование

```sh
cd clickpipe-rs
cargo test
```

---

## 🤝 Вклад в проект

Планируете что-то добавить? Смотрите [Roadmap](#-roadmap) и выбирайте открытую задачу.  
PR и issue приветствуются!

---

## 📄 Лицензия

MIT
