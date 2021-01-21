#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct NodeClient NodeClient;

typedef struct {
  char *tip_1;
  char *tip_2;
  uint32_t error;
} GetTipsResponse;

NodeClient *client_new(const char *s);

void client_free(NodeClient *ptr);

void get_tips_response_free(GetTipsResponse *ptr);

GetTipsResponse *client_get_tips(NodeClient *ptr);
