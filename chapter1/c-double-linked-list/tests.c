#include "CuTest.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include "dllists.h"

// Finding things at the beginning, end and middle of a list
void TestInsertFind(CuTest *tc) {
    Element *head = NULL;  
    head = insert(head, "Planet");
    head = insert(head, "Earth");
    head = insert(head, "Mars");
    head = insert(head, "Moon");

    Element *a1 = find(head, "Planet");
    CuAssertPtrNotNull(tc, a1);

    Element *a2 = find(head, "Earth");
    CuAssertPtrNotNull(tc, a2);

    Element *a3 = find(head, "Moon");
    CuAssertPtrNotNull(tc, a3);

    Element *no = find(head, "Jupiter");
    CuAssertPtrEquals(tc, NULL, no);
}

// Empty list tests
void TestFindEmpty(CuTest *tc) {
    Element *head = NULL;  
    Element *no = find(head, "Jupiter");
    CuAssertPtrEquals(tc, NULL, no);
}

// Test print list
void TestPrintList(CuTest *tc) {
    Element *head = NULL;  
    head = insert(head, "Vietnam");
    head = insert(head, "Morning");
    head = insert(head, "Good");

    char *actual = format_to_string(head);
    CuAssertStrEquals(tc, "Good, Morning, Vietnam", actual);

    free(actual);
}

CuSuite* DLListSuite() {
    CuSuite* suite = CuSuiteNew();
    SUITE_ADD_TEST(suite, TestInsertFind);
    SUITE_ADD_TEST(suite, TestFindEmpty);
    SUITE_ADD_TEST(suite, TestPrintList);
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
