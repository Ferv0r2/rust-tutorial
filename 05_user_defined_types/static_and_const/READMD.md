## Static과 Const 비교

| Attribute                            | Static                | Const     |
| ------------------------------------ | --------------------- | --------- |
| 메모리 상에 주소가 있는가            | O                     | X(inline) |
| 프로그램이 수행되는 동안 살아 있는가 | O                     | X         |
| 변경 가능한가                        | O(안전하지 않음)      | X         |
| 컴파일시 그 값이 결정되는가          | O(컴파일 시 초기화됨) | O         |
| 사용되는 곳에 인라인 되는가          | X                     | O         |
