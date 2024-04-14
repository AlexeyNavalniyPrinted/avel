FROM python:slim

WORKDIR /app

COPY main.py .

RUN pip install psycopg2-binary && \
    apt-get update && \
    apt-get install -y cron && \
    rm -rf /var/lib/apt/lists/*

COPY cron /etc/cron.d/cronjob

RUN chmod 0644 /etc/cron.d/cronjob

RUN crontab /etc/cron.d/mycronjob

RUN touch /var/log/cron.log

CMD cron && tail -f /var/log/cron.log