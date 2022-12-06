# grpc을 활용한 채팅 만들기

## gRPC

- 마이크로서비스

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

- 첫번쨰 줄은 이 파일이 proto3문법을 사용하고 있음을 암시
- message 는 세개의 필드를 정의
- 필드는 저마다 고유한 번호를 가지고 있는다.
- 인코딩시 필드 번호는 1~15는 1바이트,
- 16~2047까지는 2바이트 사용
- 자주발생하는 메세지는 1~15범위에 할당
