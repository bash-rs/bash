#ifndef CORE_RS_ALIAS_H
#define CORE_RS_ALIAS_H

#include <stdbool.h>
#ifdef __cplusplus
extern "C" {
#endif

// Add or update an alias. Both arguments must be valid null-terminated strings.
void alias_add(const char *name, const char *value);

// Get the value of an alias. Returns a newly allocated string (must be freed by the caller), or NULL if not found.
char *alias_get_value(const char *name);

// Remove an alias. Returns true if the alias was removed, false if not found.
bool alias_remove(const char *name);

// Delete all aliases.
void alias_delete_all(void);

char **alias_list(void);
void alias_list_free(char **list);

#ifdef __cplusplus
}
#endif

#endif // CORE_RS_ALIAS_H
