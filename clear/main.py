import psycopg2
from datetime import datetime, timedelta


db_params = {
    "dbname": "default",
    "user": "roach",
    "password": "password",
    "host": "cockroach-cockroachdb-public.default.svc.cluster.local",
    "port": "26257"
}

conn = psycopg2.connect(**db_params)
cursor = conn.cursor()

conn = psycopg2.connect(**db_params)
cursor = conn.cursor()

query = "DELETE FROM links WHERE last_modified < NOW() - INTERVAL '3 days';"

cursor.execute(query)

cursor.close()
conn.close()