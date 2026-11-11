#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Node Node;

typedef struct System System;

typedef struct NodeId {
  uintptr_t index;
  uint32_t generation;
} NodeId;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct Node *frameflow_get_node(struct System *system, struct NodeId id);

struct System *frameflow_init_system(void);

void frameflow_destroy_system(struct System *system);

void frameflow_compute_layout(struct System *system, struct NodeId parent);

struct NodeId frameflow_add_generic(struct System *system, struct NodeId parent);

struct NodeId frameflow_add_center(struct System *system, struct NodeId parent);

void rs_greet(const char *name);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
