# grpc을 활용한 채팅 만들기

## gRPC

- Google에서 develop 한 system이며 모든 환경에서 실행할수 있는 고성능 RPC프레임워크
- 마이크로서비스

## 구성

- protobuf
- server
- client

## 장점

- 다양한 언어를 기반으로 만들수 있다.
- 런타임환경과 개발환경을 구축
- http/2기반으로 통신 양방향 스트리밍 가능
- json 기반보다 가볍고 통신속도가 더빠르다.
- 다양한 언어 ,플랫폼을 지원
- 양방향 스트리밍 가능

## 단점

- 브라우저에서 gRPC서비스 직접 호출이 불가능(proxy)
- 안정성확보에 코드를 더짜야한다.REST API보다 안정성이 떨이지기떄문에
- 개발프로세스의 복잡도 증가,유지보수의 어려움
- rest api와는 다르게 메세지가 바이너리로 전달되기 떄문에 테스트가 어려움

## 구조

- Application Layer
- Framework Layer
- Transport Layer

## 채널

- 여러 서브 채널을 열어서 통신
- 이 채널은 재사용함으로써 통신비용을 절약할수 있다.

## 비교

- gRPC-web: GRPC로만 구성
- REST:GRPC섞어서구성하거나 REST만사용

## 워크플로우

- 1.protobufs definitions
- 2.protoc complier
- 3.Generate code
- 4.implement

## RPC

- 프로세스간 통신의 한형태
- 분산되어 있는 컴퓨터간의 통신을 위한 기술

## 마이크로서비스

- 소프트웨어가 잘 정의된 API를 통신하는 소규모의 독립적인 서비스로 구성되어 있는 경우의 소프트웨어 개발을 위한 아키텍쳐 및 조직적 접근 방식
- 에플리케이션의 확장을 용이하게 하고 개발속도를 앞당겨 혁신을 실현하고 새로운 기능의 출시 시간을 단축할수 있게 한다.

## Stub

- proto request,proto response
- RPC의 핵심개념
- 객체를 메세지로 변환 /역변환하는 레이어
- 클라이언트의 스텁과 서버의 스텁으로 나뉨
- 클라이언트의 스텁은 함수 호출에서 파라미터의 변환 및 함수 실행 후 서버에서 전달된 결과의 변환담당
- 서버의 스텁은 클라이언트가 전달한 매개 변수의 역변환 및 함수 실행결과를 담당

## Redis

- 빠른 데이터베이스

## tonic

- grpc 프레임워크
- protobuf정의를 위한 서버 및 클라이언트 코드 작성 가능

## protobuf

- 구글에서 개발하고 오픈소스로 공개한, 직렬화 데이타 구조
- 직렬화란 데이타를 파일로 저장하거나 또는 네트워크로 전송하기 위하여 바이너리 스트림 형태로 저장하는 행위
- 하나의 파일에 최대 64M까지 지원
- JSON 파일을 프로토콜 버퍼 파일 포맷으로 전환이 가능하고, 반대로 프로토콜 버퍼 파일도 JSON으로 전환이 가능
- protobuf통신시에 값이 default값과 같다면 통신시 전송하지 않는다.
- 주로 Rest서버로 POST에서 RequestBody에 Protobuf모델이 담겨서 통신하게 된다.

## 0.Syntax

```proto
syntax="proto3";
```

- **syntax**에는 proto2와 proto3가 있다.
- 해당 내용이 없으면 자동으로 proto2적용
- **gRPC를 활용하려면 proto3를 사용**해야 한다.

## 1.메세지 타입

```proto
syntax = "proto3";
/*
int32,int64:음수에 비효율적
uint32,uint64
sint32,sint64
fixed32,fixed64
sfixed32,sfixed64
bool
string
bytes

**위 값들의 default**
numeric:0
bool:false
string:null
byte:null
*/
message SearchRequest {
  string query = 1;//문자형 필드
  int32 page_number = 2;//정수형필드
  int32 result_per_page = 3;
}
```

- message 는 세개의 필드를 정의

## 2.

```proto
message SearchRequest {
required  string query = 1;//문자형 필드
optional int32 page_number = 2;//정수형필드
repeated  int32 result_per_page = 3;
}
/*
proto3에서는 required,optional deprecate됨
*/
```

## 3.필드번호

```proto
message SearchRequest {
reserved  3,9,5 to 7;
reserved "leo","dabin";

}
/*
기존 사용하던 필드를 중간에 제거,이후 다시 해당 필드를 재사용하게 되면 일관성 및 호환성에 장애를 초래
reserved 미리 예약

*/
```

- 필드는 저마다 고유한 번호를 가지고 있는다.
- 인코딩시 필드 번호는 1~15는 1바이트,
- 16~2047까지는 2바이트 사용
- 자주발생하는 메세지는 1~15범위에 할당

## 4.Service

```proto
service Chat {
  rpc Login(LoginRequest) returns (Token) {}
}
```

- Service는 RPC를 통해 서버가 클라이언트가 제공할 함수의 형태를 정의한다.
- 서비스명과 RPC메소드명 모드 CamleCase형태권장
- stream옵션을 주면 양방향 streamingRPC를 구현할수 있다.

## 당근마켓

- 이미지 같은 서비스
- 쿠버네티스
- protobuf를 이용해 json보다 좋은 성능
- 통신비용 절감
- 내장기능 풍부(인증)

- gprpc클라이언트와 서버

- 채널은 사용자가 표면적으로 손쉽게 메세지를 보낼수있는 쉬운 인터페이스 제공
- 채널은 하나의 엔드포인트에 대한 virtual connection을 repesent한다.
- 클라이언트가 gRPC채널을 만들면 내부적으로 서버와 http2 conn
- RPC 는 HTTP/2의 stream으로 처리
- Message는 HTTP/2의 frame으로 처리
- grpc클라이언트 resolver ,LB(로드밸런스)를 들고있음
- 리졸브는 주기적으로 target DNS를 리졸브하면서 엔드포인터들을 통신
- conn이 실패하면 LB는 직전에 사용했던 address list 를 사용해서 재연결 싲가

- Keepalive
- KeepAlive는 HTTP/2ping 프레임을 보내 연결 상태를 주기적으로 확인
- 실제로 커넥션을 살리는 역활

## 예시

- 백서버는 golang
- 각각의 서버는python
