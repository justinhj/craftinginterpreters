#include "CuTest.h"
#include <stdio.h>
#include <string.h>
#include <ctype.h>

#include "dllists.h"

void TestStrToUpper(CuTest *tc) {
    Element *head = NULL;  
    head = insert(head, "you");
    head = insert(head, "are");
    head = insert(head, "How");
    head = insert(head, "World");
    head = insert(head, "Hello");

    Element *actual = find(head, "World");
    CuAssertPtrNotNull(tc, actual);

    Element *actual2 = find(head, "Moon");
    CuAssertPtrEquals(tc, NULL, actual2);
}

CuSuite* DLListSuite() {
    CuSuite* suite = CuSuiteNew();
    SUITE_ADD_TEST(suite, TestStrToUpper);
    return suite;
}

void RunAllTests(void) {
    CuString *output = CuStringNew();
    CuSuite* suite = CuSuiteNew();
    
    CuSuiteAddSuite(suite, DLListSuite());

    CuSuiteRun(suite);
    CuSuiteSummary(suite, output);
    CuSuiteDetails(suite, output);
    printf("%s\n", output->buffer);
}

int main(void) {
    RunAllTests();
}
