import requests
from bs4 import BeautifulSoup

aws_url = "https://instances.vantage.sh/"


response = requests.get(aws_url)

soup = BeautifulSoup(response.text, "html.parser")
print(soup)