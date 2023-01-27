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
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

