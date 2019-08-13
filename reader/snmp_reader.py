import logging
logger = logging.getLogger(__name__)

from easysnmp import Session
import builtins

class Reader(object):
    def __init__(self, host, port=161, community='public', version=2, timeout=60):

        self._host = host
        self._port = port
        self._community = community
        self._version = version
        self._timeout = timeout

    def __enter__(self):
        # Create an SNMP session to be used for all our requests
        self._session = Session(hostname=self._host, remote_port=self._port, timeout=self._timeout,
            community=self._community, version=self._version)

        return self

    def __exit__(self, type, value, traceback):
        pass

    def read(self, oid, **kwargs):
        snmpval = self._session.get(oid)
        val = snmpval.value

        value = self.__process(val, **kwargs)

        return value

    def __process(self, val, **rdg):

        # Functions to call based on defined datatype
        funcs = {
            'int16':  int,
            'int32':  int,
            'int':    int,
            'float':  float,
            'single': float,
            'double': float
        }

        if 'datatype' in rdg and rdg['datatype'] in funcs:
            value = funcs[rdg['datatype']](val)
        else:
            value = val

        if rdg.get('multiplier'):
            value = value * rdg['multiplier']

        # Apply an offset if desired
        if rdg.get('offset'):
            value = value + rdg['offset']

        return value