<?xml version="1.0" encoding="UTF-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
<xsl:output method="xml" encoding="UTF-8" omit-xml-declaration="yes" indent="no" />

<xsl:variable name="nl">
	<xsl:text>&#13;&#10;</xsl:text>
</xsl:variable>
<xsl:variable name="brnl">
	<br /><xsl:copy-of select="$nl" />
</xsl:variable>

<xsl:variable name="HtmlHeader">
	<xsl:text disable-output-escaping="yes"><![CDATA[<!DOCTYPE html>]]></xsl:text>
		<xsl:copy-of select="$nl" />
	<xsl:text disable-output-escaping="yes"><![CDATA[<html xmlns="http://www.w3.org/1999/xhtml">]]></xsl:text>
		<xsl:copy-of select="$nl" />
</xsl:variable>

<xsl:variable name="HtmlFooter">
	<xsl:text disable-output-escaping="yes"><![CDATA[</html>]]></xsl:text>
		<xsl:copy-of select="$nl" />
</xsl:variable>

<!-- Design Copyright (C) 2003-2021 Dominik Reichl -->
<xsl:variable name="DocStyle">
<xsl:text disable-output-escaping="yes"><![CDATA[<style type="text/css">
/* <]]><![CDATA[![CDATA[ */
body, kbd kbd {
	font-family: Verdana, Arial, sans-serif;
	font-size: 13px;
}

body {
	color: #000000;
	background-color: #FFFFFF;
}

p {
	margin-left: 0px;
}

h2 {
	font-size: 18px;
	font-weight: bold;
}

th, td {
	text-align: left;
	vertical-align: top;
}

a {
	color: #0000DD;
	text-decoration: none;
}

a:hover, a:active {
	color: #6699FF;
	text-decoration: underline;
}

table.tablebox {
	background-color: #EEEEEE;
	padding: 0px 0px 0px 0px;
	empty-cells: show;
	width: 100%;
	margin: 0px 0px 0px 0px;

	table-layout: fixed;
}

table.tablebox, table.tablebox tr th, table.tablebox tr td {
	border: 1px solid #AFB5CF;
	border-collapse: collapse;
}

table.tablebox tr th {
	padding: 2px 5px 2px 5px;
	background-color: #EEEEEE;
	background-image: -webkit-linear-gradient(top, #C1C1C1, #EEEEEE);
	background-image: -moz-linear-gradient(top, #C1C1C1, #EEEEEE);
	background-image: -ms-linear-gradient(top, #C1C1C1, #EEEEEE);
	background-image: linear-gradient(to bottom, #C1C1C1, #EEEEEE);
	font-weight: bold;
}

table.tablebox tr td {
	padding: 5px 5px 5px 5px;
	background-color: #F0F0F0;

	overflow-wrap: break-word;
	word-wrap: break-word;
}
/* ]]]]><![CDATA[> */
</style>
]]></xsl:text>
</xsl:variable>

<xsl:template match="/">
	<xsl:copy-of select="$HtmlHeader" />
	<head>
		<meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />
		<title>KDBX Common</title>
	</head><xsl:copy-of select="$nl" />
	<body>
		<p>The <em>KDBX_Common.xsl</em> stylesheet is not supposed to be used directly.</p>
	</body><xsl:copy-of select="$nl" />
	<xsl:copy-of select="$HtmlFooter" />
</xsl:template>

</xsl:stylesheet>
