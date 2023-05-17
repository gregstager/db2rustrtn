/**********************************************************************
** (c) Copyright IBM Corp. 2007 All rights reserved.
** 
** The following sample of source code ("Sample") is owned by International 
** Business Machines Corporation or one of its subsidiaries ("IBM") and is 
** copyrighted and licensed, not sold. You may use, copy, modify, and 
** distribute the Sample in any form without payment to IBM, for the purpose of 
** assisting you in the development of your applications.
** 
** The Sample code is provided to you on an "AS IS" basis, without warranty of 
** any kind. IBM HEREBY EXPRESSLY DISCLAIMS ALL WARRANTIES, EITHER EXPRESS OR 
** IMPLIED, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF 
** MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. Some jurisdictions do 
** not allow for the exclusion or limitation of implied warranties, so the above 
** limitations or exclusions may not apply to you. IBM shall not be liable for 
** any damages you suffer as a result of using, copying, modifying or 
** distributing the Sample, even if IBM has been advised of the possibility of 
** such damages.
***********************************************************************/

// In numerous cases, we are following Db2 naming conventions, so
// turn off these warnings.
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#![allow(unused_variables)]
#![allow(dead_code)]

use std::os::raw::{c_char,c_short};


// These are the #defines for types from sqludf.h

//#define   SQLUDF_SMALLINT    short                /* for SMALLINT             */
//#define   SQLUDF_INTEGER     sqlint32             /* for INTEGER              */
//#define   SQLUDF_BIGINT      sqlint64             /* for BIGINT (64 bit)      */
//#define   SQLUDF_DOUBLE      double               /* for DOUBLE PRECISION     */
//#define   SQLUDF_REAL        float                /* for REAL                 */
//#define   SQLUDF_CHAR        char                 /* for CHAR with/without FOR BIT DATA */
//#define   SQLUDF_BINARY      char                 /* for BINARY               */
//#define   SQLUDF_VARCHAR     char                 /* for VARCHAR              */


//#define SQLUDF_SQLSTATE_LEN     5      /* length of SQLSTATE                  */
//#define SQLUDF_SCRATCHPAD_LEN   100    /* length of scratchpad                */
//#define SQLUDF_FQNAME_LEN       517    /* length of fully-qualified name      */
//#define SQLUDF_SPECNAME_LEN     128    /* length of function specific name    */
//#define SQLUDF_MSGTEXT_LEN      1000   /* length of message text area         */
//#define SQLUDF_MAX_IDENT_LEN    128    /* max length of identifier            */
//#define SQLUDF_SH_IDENT_LEN     8      /* length of short identifier          */


//#define SQLUDF_TRAIL_ARGS char sqludf_sqlstate[SQLUDF_SQLSTATE_LEN+1],  \
//                          char sqludf_fname[SQLUDF_FQNAME_LEN+1],       \
//                          char sqludf_fspecname[SQLUDF_SPECNAME_LEN+1], \
//                          char sqludf_msgtext[SQLUDF_MSGTEXT_LEN+1]


// We could define SQLUDF_INTEGER as a c_int, but that would not
// be valid on all older platforms.  Use equivalent rust type instead.
type SQLUDF_SMALLINT = c_short;
type SQLUDF_INTEGER = i32;

// A C-compatible function that doubles the input.
#[no_mangle]
pub extern "C" fn DoubleExample
(
   inNumToDouble : * const SQLUDF_INTEGER,
   outDoubled    : * mut   SQLUDF_INTEGER,
   numToDoubleNullInd : * const SQLUDF_SMALLINT,
   doubledNullInd : * mut SQLUDF_SMALLINT,
   sqludf_sqlstate : * mut c_char,
   sqludf_fname : * mut c_char,
   sqludf_fspecname : * mut c_char,
   sqludf_msgtext : * mut c_char,
)
{
    unsafe
    {
        if *numToDoubleNullInd == -1
        {
            *doubledNullInd = -1;
            return
        }

        *outDoubled = (*inNumToDouble) * 2;

        *doubledNullInd = 0;
    }
}