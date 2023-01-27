--
-- PostgreSQL database dump
--

-- Dumped from database version 14.5
-- Dumped by pg_dump version 14.5

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    cid integer NOT NULL,
    cname text NOT NULL,
    cage integer NOT NULL,
    cemail character(50),
    cmobile text,
    staff_id integer,
    data_id integer
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    dataid integer NOT NULL,
    datasize character(10) NOT NULL,
    dataduration integer,
    dataprice integer
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_id integer NOT NULL,
    dno integer NOT NULL,
    dname text NOT NULL,
    dlocation text,
    pno integer
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer NOT NULL,
    pname text NOT NULL,
    pduration text NOT NULL,
    project_id integer
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name text NOT NULL,
    dno integer NOT NULL,
    age integer,
    mobile character varying(15),
    staff_sal integer
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (cid, cname, cage, cemail, cmobile, staff_id, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com                                 	08055089112	102	5
111	Lilian Jaiya	43	l_jaiye@gmail.com                                 	08055185341	100	3
112	Arthur Musa	50	a_musa@gmail.com                                  	07055282813	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com                                	09052356772	100	2
114	Marviene Mapa	33	m_mapa@gmail.com                                  	08053333551	120	5
115	Oghenero Agor	50	o_agor@gmail.com                                  	07055566774	117	11
116	Adams Bree	33	a_bree@gmail.com                                  	08056765424	102	1
117	Okafor Mathias	45	o_mathias@gmail.com                               	08056763367	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com                               	07056774423	117	11
119	Lawal Tamire	35	l_tamire@gmail.com                                	09052111101	107	5
120	James Job	44	j_job@gmail.com                                   	08059693919	100	8
121	Matthew Jakande	21	m_jakande@gmail.com                               	07051232144	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com                              	08054921923	107	5
\.


--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (dataid, datasize, dataduration, dataprice) FROM stdin;
1	350MB     	2	200
2	1.8GB     	14	500
3	3.9GB     	30	1000
4	7.5GB     	30	1500
5	9.2GB     	30	2000
6	10.8GB    	30	2500
7	14GB      	30	3000
8	18GB      	30	4000
9	24GB      	30	5000
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_id, dno, dname, dlocation, pno) FROM stdin;
108	1	Administration	Ikeja	44
101	2	Account	Egbeda	11
100	3	Packaging	Ajah	44
120	4	Research	VI	33
97	5	Account	Magodo	22
122	6	Operations	Mile 2	44
107	7	Packaging	Ketu	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_id) FROM stdin;
11	A	9 Months	102
22	B	14 Months	97
33	C	16 Months	120
44	D	25 Months	108
55	E	9 Months	107
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, age, mobile, staff_sal) FROM stdin;
101	Alade Joy	2	33	8023089832	250000
100	Mustapha Ali	3	32	8063285831	175000
107	Alokwe Martin	7	48	7090082812	380000
97	Dankade Aminat	5	40	9023688832	550000
108	Josiah Joshua	1	30	8053189131	120000
102	Mankinde Mary	2	55	9023487830	450000
122	Osahon Mark	6	44	8022289842	320000
117	Suleman Ajayi	3	50	7030089811	800000
104	Kuti Lawal	1	35	9145689842	750000
120	Adeleke Jane	4	38	7061045862	200000
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (cid);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (dataid);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_id);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (pno);


--
-- PostgreSQL database dump complete
--

