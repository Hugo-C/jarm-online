FROM python:3.11

RUN pip install --upgrade pip

WORKDIR /code

ADD ./requirements.txt /code/requirements.txt

RUN pip install -r requirements.txt

RUN playwright install && playwright install-deps

ENTRYPOINT ["pytest"]