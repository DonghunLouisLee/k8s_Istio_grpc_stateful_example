# Introduction


오늘 회의에서 말씀해주신것 같이 에러 발생시 복구가 저희 서비스 특성상 매우 중요한것 같습니다. 또한 에러 발생시 대처 방안이 잘 수립되어 있어야 실제로 코드를 쓸때도 고민을 덜 할수있으므로 아래와 같이 정리해보았습니다. 아직 리시버랑 인퍼런스의 관계가 정확히 정립되지 않아 유저, 엔드포인트, 매니저 세 개에서 발생할 수 있는 에러와 대응방안만 적었습니다. 

# Error handling 및 데이터베이스 대전제

1. 디비에 기록해야 하는 주체가 다운될수 있기 때문에 디비에는 에러를 기록하지 않고 오직 성공적인 액션만 기록한다.

2. 시간은 각 팟마다 동일하다고 보장을 할 수 없기 때문에 시간을 기준으로 액션을 취하지 않는다.

3. 엔드포인트는 state를 갖지 않고 최소한의 정보만 가지고 있는다. 

4. 엔드포인트가 매니저한테 커넥션을 할때 어떤 매니저로 갈 지 특정할 수 없다. 

5. 엔드포인트 입장에서는 user connection failure나 endpoint 사망이나 똑같다.

6. 매니저 디비에 이벤트를 로깅할때 메세지를 보내는거랑 메세지 수신 확인 메세지 모두 기록해야 합니다. 수신 확인 로그를 사용하면 조금의 중복 데이터는 생길 수 있지만 데이터 로스가 없는 conservative recovery 가 가능합니다.

# 에러 대처방안

에러가 발생할 수 있는 상황은 user connection failure, endpoint pod failure, manager pod failure로 나눌수 있습니다. 그리고 아래에 자세히 설명을 적어놓았는데 이해를 돕기 위해 간단한 예시와 함께 설명해보겠습니다. 

### Example
We have one user called "louis" who wants to register 3 jobs. 
Our current configuration is 2 endpoints and 3 servers. And for the sake of simplicity, I'll represent the status as below

Status representation: (a, b, c) = (job_id, endpoint_id, manager_id)

So let's assume our current status is: 
(1, 1, 1),
(2, 2, 2),
(3, 1, 3)

## 1. User connection failure
1. job1 connection was lost from the user side(현실적으로는 빠른시간 내에 유저가 재접속을 시도할거임)

    => endpoint_1 should destroy the socket channel completely(사실 자동으로 됨)

    => endpoint_1 should tell manager_1 to destory the job_1 in the memory

    => manager_1 makes an event log: endpoint connection failure and destroys job_1 from the memory

2. job1 reconnected to endpoint_2 few seconds later

    => endpoint_2 checks job_id and makes connection to any of the managers(let's say manager2)

    => manager_2 should recreate the job1 based on the event logs and send endpoint_2 ok sign once done

    => continue the work

__Current status: (1,2,2), (2,2,2), (3,1,3)__

## 2. Endpoint pod failure

1. endpoint_1 fails so job3 is affected

    => user receives socket connection failure and will try to reconnect soon

    => manager_3 receives grpc connection error and makes an event log(endpoint connection failed) and destroys job_3 from the memory

2. user reconnectes to endpoint_2

    => endpoint_2 is now newly connected to manager_1 

    => manager_1 should recreate the state of job3 based on the logs and send endpoint_2 ok sign once done

    => continue the work

__Current state: (1,2,2), (2,2,2), (3,2,1)__

## 3. Manager pod failure

1. manager_2 fails at time X. Then job1(endpoint 2 ) and job2(endpoint 2 ) are affected

    => endpoints receive grpc connection failure, <그러면 먼저 유저한테 에러(로그) 알려주고> try to reconnect to a new manager using some kind of backoff algorithm (let
s assume that job1 is directed to manager1 and job2 is directed to manager3)

    => manager1 and manager2 should both recreate job1 and job2 states based on the logs

    => once done, notify endpoint2. 이때 다시 유저한테 연결 성공(로그) 메세지지를 보내줘도 되고 안 보내줘도 되고..

__Currennt state: (1, 2, 1), (2,2,3), (3,1,3)__

## Endpoint Database Structure

유저를 추가하기로 하면 직접 팟에 ssh에서 uuid로 토큰 하나를 만들고 그 토큰으로 테이블(document or anything)을 하나 만들어주는 방식으로 하면 좋을것 같습니다. (사실 이러다가 dashboard 하나는 있는게 좋을것 같기도...)
그리고 테이블을 아래 방식으로 이루어지면 좋을 것 같습니다.

columns: job_id, conneceted_endpoint_id, connceted_manager_id

## Manager database structure
