# Weather 

This is simple weather api. 
It uses 2 apis: 

[VisualCrossing](https://www.visualcrossing.com/resources/documentation/weather-api/timeline-weather-api/)
[AerisWeather](https://www.aerisweather.com/support/docs/api/reference/endpoints/forecasts/)

## Build
Put api keys into docker-compose.yaml and run:

```
docker-compose up --build
```

## Example requests
Request
```
/date_range?start_date=2021-07-02&end_date=2021-07-07&city=Moscow,RU
```
Response: 
```
[
{"temperature":19.95,"date":"2021-07-02"},{"temperature":20.25,"date":"2021-07-03"},{"temperature":22.7,"date":"2021-07-04"},{"temperature":23.0,"date":"2021-07-05"},{"temperature":22.3,"date":"2021-07-06"},{"temperature":24.05,"date":"2021-07-07"}
]
```
_______________
Request
```
/single_date?date=2021-07-02&city=Moscow,RU
```
Response:
```
{"temperature":19.95,"date":"2021-07-02"}
```
__________________________
Request
```
/single_date?city=Moscow,RU
```
Response: 
```
{"error":{"ParamNotSpecified":"date"}}
```
_________________________
Request
```
/single_date?date=2021-07-02&city=asdasd
```
Response: 
```
{"error":{"WeatherApiError":{"Other":"\"Invalid location found. Please check your location parameter:asdasd\""}}}
```